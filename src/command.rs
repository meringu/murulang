mod build;

use build::Build;
use clap::Clap;
use std::error;

#[derive(Clap, Debug)]
pub enum SubCommand {
    Build(Build),
}

impl SubCommand {
    pub fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        match self {
            SubCommand::Build(b) => b.execute(),
        }
    }
}
