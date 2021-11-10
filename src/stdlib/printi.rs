use crate::ast::{FunctionSignature, VariableType};
use crate::{wasm, wasm::Expression, wasm_dollar};

pub fn signature() -> (&'static str, FunctionSignature) {
    (
        "printi",
        FunctionSignature {
            arg_types: vec![],
            return_type: VariableType::Int,
        },
    )
}

pub fn func() -> Expression {
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
}
