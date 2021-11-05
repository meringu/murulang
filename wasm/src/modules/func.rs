use super::local::Local;
use super::param::Param;
use super::result::Result;
use crate::expression::{Expressable, Expression};

pub struct Func {
    id: Option<String>,
    locals: Vec<Local>,
    params: Vec<Param>,
    results: Vec<Result>,
}

impl Expressable for Func {
    fn to_expression(&self) -> Expression {
        let mut l = vec![Expression::new("func")];

        if let Some(id) = &self.id {
            let mut atom = String::with_capacity(id.len() + 1);
            atom.push('$');
            atom.push_str(id);
            l.push(Expression::new(atom))
        }

        for local in self.locals.iter() {
            l.push(local.to_expression());
        }

        for param in self.params.iter() {
            l.push(param.to_expression());
        }

        for result in self.results.iter() {
            l.push(result.to_expression());
        }

        Expression::new(l)
    }
}

impl Func {
    pub fn new() -> Self {
        Self {
            id: None,
            locals: vec![],
            params: vec![],
            results: vec![],
        }
    }

    pub fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_local(mut self, local: Local) -> Self {
        self.locals.push(local);
        self
    }

    pub fn with_param(mut self, param: Param) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_result(mut self, result: Result) -> Self {
        self.results.push(result);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::result;

    #[test]
    pub fn test_empty_func() {
        assert_eq!(Func::new().to_expression().to_string(), "(func)");
    }

    #[test]
    pub fn test_empty_func_with_id() {
        assert_eq!(
            Func::new().with_id("foo").to_expression().to_string(),
            "(func $foo)"
        );
    }

    #[test]
    pub fn test_empty_func_with_locals() {
        assert_eq!(
            Func::new()
                .with_local(Local::i32())
                .with_local(Local::i32().with_id("foo"))
                .to_expression()
                .to_string(),
            "(func (local i32) (local $foo i32))"
        );
    }

    #[test]
    pub fn test_empty_func_with_results() {
        assert_eq!(
            Func::new()
                .with_result(result!(i32))
                .with_result(result!(f32))
                .to_expression()
                .to_string(),
            "(func (result i32) (result f32))"
        );
    }
}
