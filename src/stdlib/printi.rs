use crate::ast;

use wasm::{wasm, wasm_dollar, Expression};

pub fn signature() -> (&'static str, ast::FunctionSignature) {
    (
        "printi",
        ast::FunctionSignature {
            arg_types: vec![],
            return_type: ast::VariableType::Int,
        },
    )
}

pub fn func() -> wasm::SExpression {
    wasm::SExpression::Atom(
        wasm!(
            "func",
            wasm_dollar!("printi"),
            wasm!("param", wasm_dollar!("num"), "i32"),
            wasm!(
                "if",
                wasm!(
                    "i32.ne",
                    wasm!("i32.const", 0),
                    wasm!("local.get", wasm_dollar!("num"))
                ),
                wasm!(
                    "then",
                    wasm!(
                        "call",
                        wasm_dollar!("printi"),
                        wasm!(
                            "i32.div_u",
                            wasm!("local.get", wasm_dollar!("num")),
                            wasm!("i32.const", 10)
                        )
                    ),
                    wasm!(
                        "call",
                        wasm_dollar!("printc"),
                        wasm!(
                            "i32.add",
                            wasm!("i32.const", 48),
                            wasm!(
                                "i32.rem_u",
                                wasm!("local.get", wasm_dollar!("num")),
                                wasm!("i32.const", 10)
                            )
                        )
                    )
                )
            )
        )
        .to_pretty(4),
    )
}
