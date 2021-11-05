use super::func::Func;
use crate::expression::{Expressable, Expression};

pub struct Import {
    module: String,
    function: String,
    import: ImportDesc,
}

pub enum ImportDesc {
    Function(Func),
}

impl Expressable for Import {
    fn to_expression(&self) -> Expression {
        Expression::new(vec![
            Expression::new("import"),
            Expression::new(&self.module).quote(),
            Expression::new(&self.function).quote(),
            match &self.import {
                ImportDesc::Function(f) => f.to_expression(),
            },
        ])
    }
}

impl Import {
    pub fn function<S: Into<String>>(module: S, function: S, func: Func) -> Self {
        Self {
            module: module.into(),
            function: function.into(),
            import: ImportDesc::Function(func),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::param::Param;
    use super::super::result::Result;
    use crate::param;
    use crate::result;

    #[test]
    pub fn test_import_wasi_fd_write() {
        assert_eq!(
            Import::function(
                "wasi_unstable",
                "fd_write",
                Func::new()
                    .with_id("fd_write")
                    .with_param(param!(i32, i32, i32, i32))
                    .with_result(result!(i32))
            )
            .to_expression()
            .to_string(),
            "(import \"wasi_unstable\" \"fd_write\" (func $fd_write (param i32 i32 i32 i32) (result i32)))"
        );
    }
}
