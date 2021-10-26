mod printc;
mod printi;

use crate::ast::FunctionSignature;

use std::collections::HashMap;

pub fn signatures() -> HashMap::<&'static str, FunctionSignature> {
    HashMap::<&'static str, FunctionSignature>::from([
        printc::signature(),
        printi::signature(),
    ])
}

pub fn wat() -> Vec<String> {
    vec!(
        printc::wat(),
        printi::wat(),
    )
}
