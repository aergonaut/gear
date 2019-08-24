use crate::commands::Command;
use std::error::Error;
use structopt::StructOpt;

mod commands;
mod config;
mod util;

#[derive(StructOpt)]
#[structopt(name = "gear", about = "A developer QOL tool")]
struct Gear {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[structopt(flatten)]
    log: clap_log_flag::Log,
    #[structopt(subcommand)]
    cmd: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    #[structopt(name = "up", about = "Install all requirements for the project")]
    Up,
    #[structopt(name = "pr", about = "Open a Pull Request for the current branch")]
    PullRequest {
        #[structopt(
            short = "b",
            long = "base",
            help = "Optional. The base branch for the PR. If not specified, inferred from the head branch name via pattern matching (configurable)."
        )]
        base: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let program = Gear::from_args();
    program.log.log_all(Some(program.verbose.log_level()))?;
    match program.cmd {
        Subcommand::Up => commands::Up::new().run().unwrap(),
        Subcommand::PullRequest { base } => commands::PullRequest::new(base).run().unwrap(),
        _ => log::info!("Command not implemented"),
    };

    Ok(())
}
