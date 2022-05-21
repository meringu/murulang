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
                    s: "source not a .muru file".to_string(),
                }));
            }
        };

        let output: String = match &self.output {
            Some(o) => o.to_string(),
            None => format!("{}.wasm", &source[..source.len() - ".muru".len()]),
        };

        let wast = format!(
            "{}.wast",
            match output.strip_suffix(".wasm") {
                Some(s) => s,
                None => {
                    return Err(Box::new(err::StandardError {
                        s: "output not a .wasm file".to_string(),
                    }));
                }
            }
        );

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

        let wasm = program.to_wasm(stdlib::funcs())?;

        if log::Level::Debug <= level_filter {
            println!("wast:\n{}", wasm.to_pretty(4));
        }

        let pretty = wasm.to_pretty(4);
        let mut file = std::fs::File::create(std::path::Path::new(&wast))?;
        file.write_all(&pretty.as_bytes())?;

        let bin = wasm.to_bin()?;

        file = std::fs::File::create(std::path::Path::new(&output))?;
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
