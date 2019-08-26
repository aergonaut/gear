use super::Command;
use crate::errors::Result;
use log::info;

pub struct Up;

impl Up {
    pub fn new() -> Up {
        Up
    }
}

impl Command for Up {
    fn run(self) -> Result<()> {
        info!("Unimplemented!");
        Ok(())
    }
}
