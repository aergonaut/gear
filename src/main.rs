//! `gear` CLI

#![deny(missing_docs)]

use crate::commands::Command;
use structopt::StructOpt;

pub mod commands;
mod config;
mod errors;
mod util;

#[derive(StructOpt)]
#[structopt(name = "gear", about = "A developer QOL tool")]
struct Gear {
    #[structopt(subcommand)]
    cmd: Subcommand,
}

#[derive(StructOpt)]
struct CommonOpts {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[structopt(flatten)]
    log: clap_log_flag::Log,
}

#[derive(StructOpt)]
enum Subcommand {
    #[structopt(
        name = "pr",
        about = "Open a Pull Request for the current branch",
        long_about = r#"Open a Pull Request for the current branch.

By default, this command will try to infer the base branch for your PR from the name of your current HEAD, and then open your browser to a GitHub URL where you can finish opening the Pull Request.

You can override the base or head branch with options, and you can choose to print the URL to your terminal instead of opening it. See the detailed help for more information.
"#
    )]
    PullRequest {
        #[structopt(flatten)]
        opts: CommonOpts,
        #[structopt(
            short = "b",
            long = "base",
            help = "Optional. The base branch for the PR.",
            long_help = r#"Optional. The base branch for the PR.

If not specified, inferred from the head branch name via pattern matching, or `master`."#
        )]
        base: Option<String>,
        #[structopt(
            short = "H",
            long = "head",
            help = "Optional. The head branch for the PR. [default: HEAD]"
        )]
        head: Option<String>,
        #[structopt(long = "print", help = "Print URL instead of opening.")]
        print: bool,
        #[structopt(long = "host", help = "Optional. The base URL to GitHub.")]
        host: Option<String>,
        #[structopt(
            long = "repo",
            help = "Optional. The GitHub repository to open the PR on (e.g. Octocat/Spoon-Knife).",
            long_help = r#"Optional. The GitHub repository to open the PR on (e.g. Octocat/Spoon-Knife).

If not specified, inferred from the `origin` remote, or configuration."#
        )]
        repository: Option<String>,
    },
}

fn main() -> crate::errors::Result<()> {
    let program = Gear::from_args();
    let result = match program.cmd {
        Subcommand::PullRequest {
            opts,
            base,
            print,
            head,
            host,
            repository,
        } => {
            opts.log.log_all(Some(opts.verbose.log_level()))?;
            commands::PullRequest::new(head, base, host, repository, print).run()
        }
    };

    if let Err(error) = result {
        log::error!("{}", error);
    }

    Ok(())
}
