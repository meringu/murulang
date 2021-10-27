use crate::parser::Rule;
use crate::wat;
use crate::ast::{Line, Function, FunctionSignature};
use crate::err;
use std::collections::{HashMap, HashSet};

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::program))]
pub struct Program {
    pub lines: Vec<Line>,
    _eoi: EOI,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct EOI;

impl Program {
    pub fn to_wat(&self, included_sigs: HashMap::<&str, FunctionSignature>, included_fns: Vec<String>) -> Result<String, Box::<dyn std::error::Error>> {
        let mut functions = HashMap::<&str, Vec<&Function>>::new();
        let mut function_signatures = HashMap::<&str, FunctionSignature>::new();
        let mut validated = HashSet::<&str>::new();

        for (name, sig) in included_sigs {
            function_signatures.insert(name, sig);
            functions.insert(name, vec!());
        }

        for l in self.lines.iter() {
            match l {
                Line::Function(f) => {
                    match functions.get_mut(f.name.name) {
                        Some(fns) => fns.push(f),
                        None => {
                            functions.insert(
                                f.name.name,
                                vec!(f),
                            );
                        },
                    }
                }
                Line::FunctionSignature(s) => {
                    function_signatures.insert(s.name.name, FunctionSignature{
                        arg_types: s.types[0..s.types.len()-1].iter().map(|x| x.var_type).collect(),
                        return_type: s.types.last().unwrap().var_type,
                    });
                },
            }
        }

        let main = match functions.get("main") {
            Some(f) => &f[0],
            None => {
                return Err(Box::new(err::FunctionNotFoundError{name: "main"}));
            },
        };
        
        main.validate("", &functions, &mut function_signatures, &mut validated, &vec!())?;

        for (fname, _) in &functions {
            match function_signatures.get(fname) {
                None => eprintln!("Warning: unused function {}", fname),
                _ => {},
            }
        }

        let mut module_inner = vec!(
            wat::import(
                "wasi_unstable",
                "fd_write",
                "fd_write",
                vec!("i32", "i32", "i32", "i32"),
                Some("i32"),
            ),
            wat::memory(1),
            wat::export("memory", wat::memory(0)),
            wat::function(
                "_start",
                Some("_start"),
                None,
                None,
                vec!(
                    wat::call("printi", vec!(
                        wat::call("main", vec!())
                    )),
                    wat::call("printc", vec!(
                        wat::i32_const(10),
                    )),
                ),
            ),
        );

        for included_fn in included_fns {
            module_inner.push(included_fn);
        }

        for (fname, sig) in &function_signatures {
            let mut fns = vec!();
            for f in functions.get_mut(fname).unwrap() {
                let cond = f.wat_matches_condition();
                if cond.is_none() {
                    fns.push((cond, f));
                    break
                }
                fns.push((cond, f));
            }

            if fns.len() == 0 {
                continue;
            }

            if fns[fns.len()-1].0.is_some() {
                return Err(
                    Box::new(
                        err::StandardError{
                            s: "murulang was unable to ensure the function matches all cases",
                        }
                    )
                )
            }

            let mut inner = fns.pop().unwrap().1.to_wat(sig.return_type);
            while let Some((cond, f)) = fns.pop() {
                inner = wat::control_if(
                    Some(sig.return_type.to_wat()),
                    cond.unwrap(),
                    f.to_wat(sig.return_type),
                    Some(inner),
                );
            }

            module_inner.push(wat::function(
                fname,
                None,
                Some(sig.arg_types.iter().map(|t| t.to_wat()).collect()),
                Some(sig.return_type.to_wat()),
                vec!(inner),
            ));
        }

        Ok(wat::module(module_inner))
    }
}