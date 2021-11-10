mod printc;
mod printi;

use crate::ast::FunctionSignature;
use crate::wasm::Expression;
use std::collections::HashMap;

pub fn signatures() -> HashMap<&'static str, FunctionSignature> {
    HashMap::<&'static str, FunctionSignature>::from([printc::signature(), printi::signature()])
}

pub fn funcs() -> Vec<Expression> {
    vec![printc::func(), printi::func()]
}
