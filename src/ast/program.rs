use crate::ast::Function;
use crate::parser::Rule;
use crate::stdlib;
use crate::{wasm, wasm::Expression, wasm_dollar, wasm_quote};

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::program))]
pub struct Program<'a> {
    pub functions: Vec<Function<'a>>,
    _eoi: EOI,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct EOI;

impl<'a> Program<'a> {
    pub fn to_wasm(
        &self,
        lib: Vec<stdlib::Func>,
    ) -> Result<Expression, Box<dyn std::error::Error>> {
        let mut module = wasm!(
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

        for func in self.functions.iter() {
            module = module.extend(func.to_wasm());
        }

        for func in lib {
            module = module.extend(func.to_wasm())
        }

        Ok(module)
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
        let source_content = include_str!("../../examples/example.muru")
            .lines()
            .filter(|x| !x.starts_with("#"))
            .collect::<Vec<&str>>()
            .join("\n");
        let mut parse_tree = parser::Parser::parse(parser::Rule::program, &source_content).unwrap();
        let program = Program::from_pest(&mut parse_tree).unwrap();
        let wasm = program.to_wasm(stdlib::funcs()).unwrap();
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
