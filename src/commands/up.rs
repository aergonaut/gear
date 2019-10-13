use super::Command;
use crate::errors::Result;
use log::info;

/// Unimplemented. Install all dependencies for a project.
///
/// This command is unimplemented.
pub struct Up;

impl Up {
    pub(crate) fn new() -> Up {
        Up
    }
}

impl Command for Up {
    fn run(self) -> Result<()> {
        info!("Unimplemented!");
        Ok(())
    }
}
