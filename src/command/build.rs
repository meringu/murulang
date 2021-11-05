use ast::Program;
use clap::Clap;
use from_pest::FromPest;
use log::LevelFilter;
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

    // Log level
    #[clap(short)]
    log_level: Option<LevelFilter>,
}

impl Build {
    pub fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        let level_filter = match self.log_level {
            Some(l) => l,
            _ => LevelFilter::Info,
        };

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

        if log::Level::Debug <= level_filter {
            println!("source:\n{}", source_content);
        }

        let mut parse_tree = parser::Parser::parse(parser::Rule::program, &source_content)?;

        if log::Level::Debug <= level_filter {
            println!("parse tree:\n{:#?}", parse_tree);
        }

        let program = Program::from_pest(&mut parse_tree).unwrap();

        if log::Level::Debug <= level_filter {
            println!("ast:\n{:#?}", program);
        }

        let wasm = program.to_wasm(stdlib::signatures(), stdlib::funcs())?;

        if log::Level::Debug <= level_filter {
            println!("wat:\n{}", wasm);
        }
        let bin = wasm.bin()?;

        let mut file = std::fs::File::create(std::path::Path::new(output))?;
        file.write_all(&bin)?;

        if log::Level::Info <= level_filter {
            println!(
                "{} compiled to: {} ({} bytes)",
                self.source,
                output,
                bin.len()
            );
        }

        Ok(())
    }
}
