use super::Command;
use crate::config::ProjectConfig;
use regex::Regex;
use std::error::Error;

#[derive(Debug)]
pub struct PullRequest {
    base: Option<String>,
    config: Option<ProjectConfig>,
}

impl PullRequest {
    pub fn new(base: Option<String>) -> PullRequest {
        let config = ProjectConfig::load().ok();
        PullRequest {
            base: base,
            config: config,
        }
    }

    fn base_pattern(&self) -> Option<Regex> {
        self.config
            .as_ref()
            .and_then(|config| config.commands())
            .or_else(|| {
                log::debug!("Could not fetch commands table");
                None
            })
            .and_then(|cmds| cmds.get("pr"))
            .or_else(|| {
                log::debug!("Could not fetch pr table");
                None
            })
            .and_then(|inner| inner.as_table())
            .and_then(|pr| pr.get("base_pattern"))
            .or_else(|| {
                log::debug!("Could not fetch base_pattern");
                None
            })
            .and_then(|inner| inner.as_str())
            .and_then(|pattern| Regex::new(&pattern).ok())
            .or_else(|| {
                log::debug!("Could not compile regex");
                None
            })
    }

    fn infer_base_branch_from_head<'head>(&self, head: &'head str) -> Option<&'head str> {
        self.base_pattern()
            .or_else(|| {
                log::debug!("base_branch pattern not specified");
                None
            })
            .and_then(|re| re.captures(head))
            .or_else(|| {
                log::debug!("base_branch pattern did not match head branch");
                None
            })
            .and_then(|captures| {
                if let Some(base) = captures.name("base") {
                    log::debug!("using named $base capture from base_branch pattern");
                    Some(base.as_str())
                } else if let Some(base) = captures.get(0) {
                    log::debug!("using capture $1 from base_branch pattern");
                    Some(base.as_str())
                } else {
                    log::debug!("base_branch pattern produced no captures");
                    None
                }
            })
    }
}

impl Command for PullRequest {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let repo = git2::Repository::discover(std::env::current_dir()?)?;
        let head_ref = repo.head()?;
        if !head_ref.is_branch() {
            log::error!(
                "Current HEAD is not a branch. Cannot create Pull Request without a branch."
            );
            return Ok(());
        }

        let head_ref_name = head_ref.shorthand();
        if head_ref_name.is_none() {
            log::error!("HEAD ref name is not valid UTF-8.");
            return Ok(());
        }

        let head_branch = head_ref_name.unwrap();

        let base_branch = match self.base {
            Some(base) => base,
            None => self
                .infer_base_branch_from_head(head_branch)
                .unwrap_or("master")
                .to_owned(),
        };

        log::info!(
            "Creating Pull Request from {} into {}",
            head_branch,
            base_branch
        );

        Ok(())
    }
}
