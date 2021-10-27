use crate::ast;
use crate::wat;

// printi prints an i32 with a new break

pub fn signature() -> (&'static str, ast::FunctionSignature) {
    (
        "printi",
        ast::FunctionSignature {
            arg_types: vec!(),
            return_type: ast::VariableType::Int,
        },
    )
}

pub fn wat() -> String {
    wat::function(
        "printi",
        None,
        Some(vec!("i32".to_string())),
        None,
        vec!(
            wat::control_if(
                None,
                wat::i32_ne(wat::i32_const(0), wat::get_local(0)),
                vec!(
                    wat::call(
                        "printi",
                        vec!(
                            wat::i32_div_u(wat::get_local(0), wat::i32_const(10)),
                        ),
                    ),
                    wat::call(
                        "printc",
                        vec!(
                            wat::i32_add(
                                wat::i32_const(48),
                                wat::i32_rem_u(wat::get_local(0), wat::i32_const(10)),
                            )
                        )
                    ),
                ).join("\n"),
                None,
            )
        ),
    )
}
