use crate::commands::Command;
use structopt::StructOpt;

mod commands;
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
    #[structopt(name = "up", about = "Install all requirements for the project")]
    Up {
        #[structopt(flatten)]
        opts: CommonOpts,
    },
    #[structopt(name = "pr", about = "Open a Pull Request for the current branch")]
    PullRequest {
        #[structopt(flatten)]
        opts: CommonOpts,
        #[structopt(
            short = "b",
            long = "base",
            help = "Optional. The base branch for the PR. If not specified, inferred from the head branch name via pattern matching (configurable)."
        )]
        base: Option<String>,
        #[structopt(
            short = "H",
            long = "head",
            help = "Optional. The head branch for the PR. If not specified, inferred from the HEAD of the current Git repository."
        )]
        head: Option<String>,
        #[structopt(
            short = "c",
            long = "copy",
            help = "Optional. Instead of opening the Pull Request page in your browser, copy the URL to your clipboard instead."
        )]
        copy: bool,
    },
}

fn main() -> crate::errors::Result<()> {
    let program = Gear::from_args();
    match program.cmd {
        Subcommand::Up { opts } => {
            opts.log.log_all(Some(opts.verbose.log_level()))?;
            commands::Up::new().run().unwrap()
        }
        Subcommand::PullRequest {
            opts,
            base,
            copy,
            head,
        } => {
            opts.log.log_all(Some(opts.verbose.log_level()))?;
            commands::PullRequest::new(head, base, copy).run().unwrap()
        }
        _ => log::info!("Command not implemented"),
    };

    Ok(())
}
