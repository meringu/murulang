use super::func::Func;
use crate::expression::{Expressable, Expression};

pub struct Module {
    id: Option<String>,
    funcs: Vec<Func>,
}

impl Expressable for Module {
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

impl Module {
    pub fn new() -> Self {
        Self {
            id: None,
            funcs: vec![],
        }
    }

    pub fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_func(mut self, func: Func) -> Self {
        self.funcs.push(func);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_empty_module() {
        assert_eq!(Module::new().to_expression().to_string(), "(module)");
    }

    #[test]
    pub fn test_empty_module_with_id() {
        assert_eq!(
            Module::new().with_id("foo").to_expression().to_string(),
            "(module $foo)"
        );
    }

    #[test]
    pub fn test_example_module() {
        assert_eq!(
            Module::new()
                .with_id("foo")
                .with_func(Func::new().with_id("bar"))
                .to_expression()
                .to_string(),
            "(module $foo (func $bar))"
        );
    }
}
