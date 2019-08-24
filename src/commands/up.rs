use super::Command;
use log::info;
use std::error::Error;

pub struct Up;

impl Up {
    pub fn new() -> Up {
        Up
    }
}

impl Command for Up {
    fn run(self) -> Result<(), Box<dyn Error>> {
        info!("Unimplemented!");
        Ok(())
    }
}
