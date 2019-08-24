mod pull_request;
mod up;

pub use pull_request::PullRequest;
use std::error::Error;
pub use up::Up;

pub trait Command {
    fn run(self) -> Result<(), Box<dyn Error>>;
}
