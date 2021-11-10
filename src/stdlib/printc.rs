use crate::ast::{FunctionSignature, VariableType};
use crate::{wasm, wasm::Expression, wasm_dollar};

pub fn signature() -> (&'static str, FunctionSignature) {
    (
        "printc",
        FunctionSignature {
            arg_types: vec![],
            return_type: VariableType::Int,
        },
    )
}

pub fn func() -> Expression {
    wasm!(
        "func",
        wasm_dollar!("printc"),
        wasm!("param", wasm_dollar!("char"), "i32"),
        wasm!("i32.store", wasm!("i32.const", 0), wasm!("i32.const", 8)),
        wasm!("i32.store", wasm!("i32.const", 4), wasm!("i32.const", 2)),
        wasm!(
            "i32.store",
            wasm!("i32.const", 8),
            wasm!("local.get", wasm_dollar!("char"))
        ),
        wasm!(
            "call",
            wasm_dollar!("fd_write"),
            wasm!("i32.const", 1),
            wasm!("i32.const", 0),
            wasm!("i32.const", 1),
            wasm!("i32.const", 20)
        ),
        wasm!("drop")
    )
}
