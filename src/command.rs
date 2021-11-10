mod build;
mod run;

use build::Build;
use clap::Clap;
use run::Run;
use std::error;

#[derive(Clap, Debug)]
pub enum SubCommand {
    Build(Build),
    Run(Run),
}

impl SubCommand {
    pub fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        match self {
            SubCommand::Build(b) => b.execute(),
            SubCommand::Run(r) => r.execute(),
        }
    }
}
