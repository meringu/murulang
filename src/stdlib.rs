mod printc;
mod printi;

use crate::wasm::Expression;

pub struct Func {
    pub wasm: Expression,
}

impl Func {
    fn new(wasm: Expression) -> Self {
        Self { wasm: wasm }
    }

    pub fn to_wasm(self) -> Expression {
        self.wasm
    }
}

pub fn funcs() -> Vec<Func> {
    vec![printi::new(), printc::new()]
}
