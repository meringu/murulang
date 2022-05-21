use super::Func;
use crate::{wasm, wasm_dollar};

pub fn new() -> Func {
    Func::new(wasm!(
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
    ))
}
