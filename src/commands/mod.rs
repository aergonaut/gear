//! Subcommands of the `gear` CLI. See individual commands for more documentation.

use crate::errors::Result;

mod pull_request;
mod up;

pub use pull_request::PullRequest;
pub use up::Up;

pub(crate) trait Command {
    fn run(self) -> Result<()>;
}
