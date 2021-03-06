use super::Command;
use crate::config::ProjectConfig;
use crate::errors::Result;
use failure::format_err;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SSH_REMOTE: Regex = Regex::new("git@(?P<host>[^:]+):(?P<repo>.+).git").unwrap();
    static ref HTTP_REMOTE: Regex =
        Regex::new("https?://(?P<host>[^/]+)/(?P<repo>.+).git").unwrap();
}

/// Open a new PR on GitHub for the current branch.
///
/// This command will use your web browser to open the GitHub UI for creating a new Pull Request.
/// By default, your PR's base branch will be your repository's `master` branch. However, `gear`
/// can be configured to infer the appropriate base branch by inspecting the current branch name.
/// This is useful for workflows where PRs may be merged to multiple branches, for example if a
/// project maintains multiple active releases.
///
/// # Configuration
///
/// ## `commands.pr.base_pattern`
///
/// If this option is set to a regular expression, `gear` will use the regex to infer the base
/// branch by matching the current head branch.
///
/// The regular expression must produce one capture. It may either be the first positional capture,
/// or it may be a capture named `base`.
///
/// If the regular expression does not match or does not produce any captures, then `master` will
/// be used as the default.
///
/// # Examples
///
/// ```plain
/// $ cat gear.toml
/// [commands.pr]
/// base_pattern = '^(release/v\d+)'
/// $ git branch
/// release/v25/fix-critical-bug
/// $ gear pr --print
/// https://github.com/aergonaut/gear/compare/release/v25...release/v25/fix-critical-bug?expand=1
#[derive(Debug)]
pub struct PullRequest {
    head: Option<String>,
    base: Option<String>,
    config: Option<ProjectConfig>,
    print: bool,
    host: Option<String>,
    repository: Option<String>,
}

impl PullRequest {
    pub(crate) fn new(
        head: Option<String>,
        base: Option<String>,
        host: Option<String>,
        repository: Option<String>,
        print: bool,
    ) -> PullRequest {
        let config = ProjectConfig::load().ok();
        PullRequest {
            head: head,
            base: base,
            config: config,
            print: print,
            host: host,
            repository: repository,
        }
    }

    fn head(&self) -> Result<String> {
        match &self.head {
            Some(head) => Ok(head.clone()),
            None => self.infer_head_from_git(),
        }
    }

    fn infer_head_from_git(&self) -> Result<String> {
        let repo = git2::Repository::discover(std::env::current_dir()?)?;
        let head_ref = repo.head()?;
        if !head_ref.is_branch() {
            Err(format_err!("HEAD is not a branch"))?;
        }

        let head_branch = head_ref
            .shorthand()
            .ok_or_else(|| format_err!("HEAD ref is not valid UTF-8"))?;

        Ok(head_branch.to_owned())
    }

    fn base(&self) -> Result<String> {
        self.base
            .clone()
            .or_else(|| {
                self.head()
                    .ok()
                    .as_ref()
                    .and_then(|head| self.infer_base_branch_from_head(head))
                    .or_else(|| {
                        log::warn!("Defaulting to master as base");
                        Some("master")
                    })
                    .map(|s| s.to_owned())
            })
            .ok_or_else(|| format_err!("Could not infer base branch"))
    }

    /// Infer the base branch from the head branch. Returns `None` if the base branch could not be
    /// inferred.
    fn infer_base_branch_from_head<'head>(&self, head: &'head str) -> Option<&'head str> {
        self.base_pattern()
            .or_else(|| {
                log::warn!("base_branch pattern not specified");
                None
            })
            .and_then(|re| re.captures(head))
            .or_else(|| {
                log::warn!("base_branch pattern did not match head branch");
                None
            })
            .and_then(|captures| {
                if let Some(base) = captures.name("base") {
                    log::warn!("using named $base capture from base_branch pattern");
                    Some(base.as_str())
                } else if let Some(base) = captures.get(0) {
                    log::warn!("using capture $1 from base_branch pattern");
                    Some(base.as_str())
                } else {
                    log::warn!("base_branch pattern produced no captures");
                    None
                }
            })
    }

    /// Compiles a regex from the `base_pattern` string from the configuration file. Returns `None`
    /// if the pattern could not be fetched or the regex could not be compiled for any reason.
    fn base_pattern(&self) -> Option<Regex> {
        self.config
            .as_ref()
            .and_then(|config| config.commands())
            .or_else(|| {
                log::warn!("Could not fetch commands table");
                None
            })
            .and_then(|cmds| cmds.get("pr"))
            .or_else(|| {
                log::warn!("Could not fetch pr table");
                None
            })
            .and_then(|inner| inner.as_table())
            .and_then(|pr| pr.get("base_pattern"))
            .or_else(|| {
                log::warn!("Could not fetch base_pattern");
                None
            })
            .and_then(|inner| inner.as_str())
            .and_then(|pattern| Regex::new(&pattern).ok())
            .or_else(|| {
                log::warn!("Could not compile regex");
                None
            })
    }

    fn url(&self, base_branch: &str, head_branch: &str) -> Result<String> {
        if let Some(host) = self.host.as_ref() {
            if let Some(repo) = self.repository.as_ref() {
                return Ok(format!(
                    "https://{}/{}/{}...{}?expand=1",
                    host, repo, base_branch, head_branch
                ));
            }
        }

        let repo = git2::Repository::discover(std::env::current_dir()?)?;

        let origin_remote = repo.find_remote("origin")?;
        let origin_url = origin_remote
            .url()
            .ok_or_else(|| format_err!("`origin` URL was not valid UTF-8"))?;

        let captures = SSH_REMOTE
            .captures(origin_url)
            .or_else(|| HTTP_REMOTE.captures(origin_url))
            .ok_or_else(|| {
                format_err!("`origin` URL did not match either SSH or HTTP remote pattern.")
            })?;

        let host = captures
            .name("host")
            .map(|m| m.as_str())
            .unwrap_or("github.com");

        let repo = captures
            .name("repo")
            .map(|m| m.as_str())
            .ok_or_else(|| format_err!("Could not infer repository from `origin` remote"))?;

        let url = format!(
            "https://{}/{}/compare/{}...{}?expand=1",
            host, repo, base_branch, head_branch
        );

        Ok(url)
    }
}

impl Command for PullRequest {
    fn run(self) -> Result<()> {
        let head_branch = self.head()?;
        let base_branch = self.base()?;

        let url = self.url(&base_branch, &head_branch)?;

        if self.print {
            println!("{}", url);
        } else {
            open::that(&url)?;
            log::info!("Opening {}", url);
        }

        Ok(())
    }
}
