mod printc;
mod printi;

use crate::ast::FunctionSignature;
use crate::wasm::Expression;
use std::collections::HashMap;

pub struct Func {
    pub sig: FunctionSignature,
    pub wasm: Expression,
}

pub struct Lib<'a> {
    pub funcs: HashMap<&'a str, Func>,
}

impl<'a> Lib<'a> {
    pub fn new() -> Self {
        let mut funcs = HashMap::new();
        funcs.insert("printi", printi::new());
        funcs.insert("printc", printc::new());
        Self { funcs: funcs }
    }
}
