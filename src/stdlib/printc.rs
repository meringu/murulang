use crate::ast;

pub fn signature() -> (&'static str, ast::FunctionSignature) {
    (
        "printc",
        ast::FunctionSignature {
            arg_types: vec![],
            return_type: ast::VariableType::Int,
        },
    )
}

pub fn func() -> wasm::SExpression {
    wasm::func(
        "printc",
        None,
        Some(wasm::param(vec![wasm::Types::I32])),
        None,
        vec![
            wasm::Types::I32.store(
                wasm::Types::I32.constant("0"),
                wasm::Types::I32.constant("8"),
            ),
            wasm::Types::I32.store(
                wasm::Types::I32.constant("4"),
                wasm::Types::I32.constant("2"),
            ),
            wasm::Types::I32.store(wasm::Types::I32.constant("8"), wasm::get_local("0")),
            wasm::call(
                "fd_write",
                vec![
                    wasm::Types::I32.constant("1"),
                    wasm::Types::I32.constant("0"),
                    wasm::Types::I32.constant("1"),
                    wasm::Types::I32.constant("20"),
                ],
            ),
            wasm::drop(),
        ],
    )
}
