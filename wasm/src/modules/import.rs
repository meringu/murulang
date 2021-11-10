use super::func::Func;
use crate::expression::{Expressable, Expression};

pub struct Import<'a> {
    module: String,
    function: String,
    import: ImportDesc<'a>,
}

pub enum ImportDesc<'a> {
    Function(Func<'a>),
}

impl Expressable for Import<'_> {
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

impl<'a> Import<'a> {
    #[doc(hidden)]
    pub fn new(module: String, function: String, import: ImportDesc<'a>) -> Self {
        Self {
            module: module,
            function: function,
            import: import,
        }
    }
}

#[macro_export]
macro_rules! import_function {
    ($module: literal, $function: literal, $func: expr) => {
        $crate::Import::new(
            $module.to_string(),
            $function.to_string(),
            $crate::ImportDesc::Function($func),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::func;
    use crate::param;
    use crate::result;

    #[test]
    pub fn test_import_wasi_fd_write() {
        assert_eq!(
            import_function!(
                "wasi_unstable",
                "fd_write",
                func!("fd_write").with_param(
                    param!(i32, i32, i32, i32)).with_result(
                    result!(i32))
            )
            .to_expression()
            .to_string(),
            "(import \"wasi_unstable\" \"fd_write\" (func $fd_write (param i32 i32 i32 i32) (result i32)))"
        );
    }
}
