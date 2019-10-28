//! Subcommands of the `gear` CLI. See individual commands for more documentation.

use crate::errors::Result;

mod pull_request;

pub use pull_request::PullRequest;

pub(crate) trait Command {
    fn run(self) -> Result<()>;
}
