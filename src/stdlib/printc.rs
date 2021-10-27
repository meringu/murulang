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
            wat::i32_store(wat::i32_const(0), wat::i32_const(8)),
            wat::i32_store(wat::i32_const(4), wat::i32_const(2)),
            wat::i32_store(wat::i32_const(8), wat::get_local(0)),
            wat::call("fd_write", vec!(
                wat::i32_const(1),
                wat::i32_const(0),
                wat::i32_const(1),
                wat::i32_const(20),
            )),
            wat::drop(),
        ),
    )
}
