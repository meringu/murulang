use ast::Program;
use clap::Clap;
use from_pest::FromPest;
use log::LevelFilter;
use pest::Parser;
use std::error;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::sync::WasiCtxBuilder;

use crate::ast;
use crate::err;
use crate::parser;
use crate::stdlib;

// Build a muru program
#[derive(Clap, Debug)]
pub struct Run {
    // Path to the muru file to build
    source: String,

    // Log level
    #[clap(short)]
    log_level: Option<LevelFilter>,
}

impl Run {
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

        let bin = wasm.to_bin()?;

        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();
        let mut store = Store::new(&engine, wasi);
        let module = Module::from_binary(&engine, &bin)?;
        linker.module(&mut store, "", &module)?;
        linker
            .get_default(&mut store, "")?
            .typed::<(), (), _>(&store)?
            .call(&mut store, ())?;

        Ok(())
    }
}
