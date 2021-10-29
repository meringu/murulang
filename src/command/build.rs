use ast::Program;
use clap::Clap;
use from_pest::FromPest;
use log::{debug, log_enabled, Level};
use pest::Parser;
use std::error;
use std::io::Write;

use crate::ast;
use crate::err;
use crate::parser;
use crate::stdlib;

// Build a muru program
#[derive(Clap, Debug)]
pub struct Build {
    // Path to the muru file to build
    source: String,

    // Output file name
    #[clap(short)]
    output: Option<String>,
}

impl Build {
    pub fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        let source = match self.source.ends_with(".muru") {
            true => &self.source,
            false => {
                return Err(Box::new(err::StandardError {
                    s: "source not a .muru file",
                }));
            }
        };

        let output = match &self.output {
            Some(o) => o,
            None => &source[..source.len() - ".muru".len()],
        };

        let source_content = String::from_utf8(std::fs::read(source)?)?;

        if log_enabled!(Level::Debug) {
            debug!("source:\n{}", source_content);
        }

        let mut parse_tree = parser::Parser::parse(parser::Rule::program, &source_content)?;

        if log_enabled!(Level::Debug) {
            debug!("parse tree:\n{:#?}", parse_tree);
        }

        let program = Program::from_pest(&mut parse_tree).unwrap();

        if log_enabled!(Level::Debug) {
            debug!("ast:\n{:#?}", program);
        }

        let wasm = program.to_wasm(stdlib::signatures(), stdlib::wat())?;

        if log_enabled!(Level::Debug) {
            debug!("wat:\n{}", wasm);
        }
        let bin = wasm.bin()?;

        let mut file = std::fs::File::create(std::path::Path::new(output))?;
        file.write_all(&bin)?;

        Ok(())
    }
}
