use crate::ast::{Function, FunctionSignature, Line};
use crate::err;
use crate::parser::Rule;
use crate::{wasm, wasm::Expression, wasm_dollar, wasm_quote};
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
    pub fn to_wasm(
        &self,
        included_sigs: HashMap<&str, FunctionSignature>,
        included_fns: Vec<Expression>,
    ) -> Result<wasm::Expression, Box<dyn std::error::Error>> {
        let mut functions = HashMap::<&str, Vec<&Function>>::new();
        let mut function_signatures = HashMap::<&str, FunctionSignature>::new();
        let mut validated = HashSet::<&str>::new();

        for (name, sig) in included_sigs {
            function_signatures.insert(name, sig);
            functions.insert(name, vec![]);
        }

        for l in self.lines.iter() {
            match l {
                Line::Function(f) => match functions.get_mut(f.name.name) {
                    Some(fns) => fns.push(f),
                    None => {
                        functions.insert(f.name.name, vec![f]);
                    }
                },
                Line::FunctionSignature(s) => {
                    function_signatures.insert(
                        s.name.name,
                        FunctionSignature {
                            arg_types: s.types[0..s.types.len() - 1]
                                .iter()
                                .map(|x| x.var_type)
                                .collect(),
                            return_type: s.types.last().unwrap().var_type,
                        },
                    );
                }
            }
        }

        let main = match functions.get("main") {
            Some(f) => &f[0],
            None => {
                return Err(Box::new(err::FunctionNotFoundError { name: "main" }));
            }
        };

        main.validate(
            "",
            &functions,
            &mut function_signatures,
            &mut validated,
            &vec![],
        )?;
        for (fname, _) in &functions {
            match function_signatures.get(fname) {
                None => eprintln!("Warning: unused function {}", fname),
                _ => {}
            }
        }

        let mut module_inner = wasm!(
            "module",
            wasm!(
                "import",
                wasm_quote!("wasi_unstable"),
                wasm_quote!("fd_write"),
                wasm!(
                    "func",
                    wasm_dollar!("fd_write"),
                    wasm!("param", "i32", "i32", "i32", "i32"),
                    wasm!("result", "i32")
                )
            ),
            wasm!("export", wasm_quote!("memory"), wasm!("memory", 0)),
            wasm!("memory", 1),
            wasm!(
                "func",
                wasm_dollar!("_start"),
                wasm!("export", wasm_quote!("_start")),
                wasm!(
                    "call",
                    wasm_dollar!("printi"),
                    wasm!("call", wasm_dollar!("main"))
                ),
                wasm!("call", wasm_dollar!("printc"), wasm!("i32.const", 10))
            )
        );

        for included_fn in included_fns {
            module_inner = module_inner.extend(included_fn);
        }

        for (fname, sig) in &function_signatures {
            let mut fns = vec![];
            for f in functions.get_mut(fname).unwrap() {
                let cond = f.wat_matches_condition();
                if cond.is_none() {
                    fns.push((cond, f));
                    break;
                }
                fns.push((cond, f));
            }

            if fns.len() == 0 {
                continue;
            }

            if fns[fns.len() - 1].0.is_some() {
                return Err(Box::new(err::StandardError {
                    s: "murulang was unable to ensure the function matches all cases",
                }));
            }

            let mut inner = fns.pop().unwrap().1.to_wasm(sig.return_type);
            while let Some((cond, f)) = fns.pop() {
                inner = wasm!(
                    "if",
                    wasm!("result", sig.return_type.to_wasm()),
                    cond.unwrap(),
                    wasm!("then", f.to_wasm(sig.return_type)),
                    wasm!("else", inner)
                );
            }

            let mut func = wasm!(wasm!("func"), wasm_dollar!(fname));

            if sig.arg_types.len() > 0 {
                let mut param = wasm!("param");
                for ty in sig.arg_types.iter() {
                    param = param.extend(ty.to_wasm());
                }
                func = func.extend(param);
            }

            func = func
                .extend(wasm!("result", sig.return_type.to_wasm()))
                .extend(inner);

            module_inner = module_inner.extend(func);
        }

        Ok(module_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::stdlib;
    use from_pest::FromPest;
    use gag::BufferRedirect;
    use pest::Parser;
    use std::io::Read;
    use wasmtime::{Engine, Linker, Module, Store};
    use wasmtime_wasi::sync::WasiCtxBuilder;

    #[test]
    fn test_example() {
        let source_content = include_str!("../../examples/example.muru");
        let mut parse_tree = parser::Parser::parse(parser::Rule::program, &source_content).unwrap();
        let program = Program::from_pest(&mut parse_tree).unwrap();
        let wasm = program
            .to_wasm(stdlib::signatures(), stdlib::funcs())
            .unwrap();
        let bin = wasm.to_bin().unwrap();

        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()
            .unwrap()
            .build();
        let mut store = Store::new(&engine, wasi);
        let module = Module::from_binary(&engine, &bin).unwrap();
        linker.module(&mut store, "", &module).unwrap();
        let func = linker
            .get_default(&mut store, "")
            .unwrap()
            .typed::<(), (), _>(&store)
            .unwrap();

        let mut buf = BufferRedirect::stdout().unwrap();
        func.call(&mut store, ()).unwrap();

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();

        assert_eq!(&output[..], "1\u{0}4\u{0}\n\u{0}");
    }
}
