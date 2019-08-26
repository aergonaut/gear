use crate::errors::Result;

mod pull_request;
mod up;

pub use pull_request::PullRequest;
pub use up::Up;

pub trait Command {
    fn run(self) -> Result<()>;
}
