use crate::expression::{Expressable, Expression};

pub struct Export<'a> {
    name: String,
    export: &'a dyn Expressable,
}

impl Expressable for Export<'_> {
    fn to_expression(&self) -> Expression {
        Expression::new(vec![
            Expression::new("export"),
            Expression::new(&self.name).quote(),
            self.export.to_expression(),
        ])
    }
}

impl<'a> Export<'a> {
    #[doc(hidden)]
    pub fn new(name: String, export: &'a dyn Expressable) -> Self {
        Self {
            name: name,
            export: export,
        }
    }
}

#[macro_export]
macro_rules! export {
    ($name:literal, $export:expr) => {
        $crate::Export::new($name.to_string(), &$export)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::memory;

    #[test]
    pub fn test_export_wasi_fd_write() {
        assert_eq!(
            export!("memory", memory!(0)).to_expression().to_string(),
            "(export \"memory\" (memory 0))"
        );
    }
}
