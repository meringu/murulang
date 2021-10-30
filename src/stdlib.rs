mod printc;
mod printi;

use crate::ast::FunctionSignature;

use std::collections::HashMap;

pub fn signatures() -> HashMap<&'static str, FunctionSignature> {
    HashMap::<&'static str, FunctionSignature>::from([printc::signature(), printi::signature()])
}

pub fn funcs() -> Vec<wasm::SExpression> {
    vec![printc::func(), printi::func()]
}
