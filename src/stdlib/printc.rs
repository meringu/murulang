use super::Func;
use crate::{wasm, wasm_dollar};

pub fn new() -> Func {
    Func::new(wasm!(
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
    ))
}
