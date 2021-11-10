#[macro_use]
extern crate pest_derive;
extern crate from_pest;
#[macro_use]
extern crate pest_ast;
extern crate pest;

// extern crate wabt;

// extern crate wat;

mod ast;
mod command;
mod err;
mod parser;
mod stdlib;
mod wasm;

use crate::command::SubCommand;
use clap::{AppSettings, Clap};
use log::{debug, error, log_enabled, Level};

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Henry Muru Paenga <meringu@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn main() {
    use std::process;

    env_logger::init();
    let opts: Opts = Opts::parse();

    if log_enabled!(Level::Debug) {
        debug!("murulang executed with:\n{:#?}", opts);
    }

    match opts.subcmd.execute() {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    };
}
