use crate::ast;
use crate::wat;

pub fn signature() -> (&'static str, ast::FunctionSignature) {
    (
        "printc",
        ast::FunctionSignature {
            arg_types: vec!(),
            return_type: ast::VariableType::Int,
        },
    )
}

pub fn wat() -> String {
    wat::function(
        "printc",
        None,
        Some(vec!("i32".to_string())),
        None,
        vec!(
            wat::store(wat::const_i32(0), wat::const_i32(8)),
            wat::store(wat::const_i32(4), wat::const_i32(2)),
            wat::store(wat::const_i32(8), wat::get_local(0)),
            wat::call("fd_write", vec!(
                wat::const_i32(1),
                wat::const_i32(0),
                wat::const_i32(1),
                wat::const_i32(20),
            )),
            wat::drop(),
        ),
    )
}
