use super::func::Func;
use crate::expression::{Expressable, Expression};

pub struct Module<'a> {
    id: Option<String>,
    funcs: Vec<Func<'a>>,
}

impl Expressable for Module<'_> {
    fn to_expression(&self) -> Expression {
        let mut l = vec![Expression::new("module")];

        if let Some(id) = &self.id {
            let mut atom = String::with_capacity(id.len() + 1);
            atom.push('$');
            atom.push_str(id);
            l.push(Expression::new(atom))
        }

        for func in self.funcs.iter() {
            l.push(func.to_expression());
        }

        Expression::new(l)
    }
}

impl<'a> Module<'a> {
    #[doc(hidden)]
    pub fn new(id: Option<String>) -> Self {
        Self {
            id: id,
            funcs: vec![],
        }
    }

    pub fn with_func(mut self, func: Func<'a>) -> Self {
        self.funcs.push(func);
        self
    }
}

#[macro_export]
macro_rules! module {
    ($id: literal) => {
        $crate::Module::new(Some($id.to_string()))
    };

    () => {{
        $crate::Module::new(None)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::func;

    #[test]
    pub fn test_empty_module() {
        assert_eq!(module!().to_expression().to_string(), "(module)");
    }

    #[test]
    pub fn test_empty_module_with_id() {
        assert_eq!(module!("foo").to_expression().to_string(), "(module $foo)");
    }

    #[test]
    pub fn test_example_module() {
        assert_eq!(
            module!("foo")
                .with_func(func!("bar"))
                .to_expression()
                .to_string(),
            "(module $foo (func $bar))"
        );
    }
}
