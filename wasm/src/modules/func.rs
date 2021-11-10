use crate::{
    expression::{Expressable, Expression},
    Local, Param, Result,
};

pub struct Func<'a> {
    id: Option<String>,
    export: Option<String>,
    params: Vec<Param>,
    results: Vec<Result>,
    locals: Vec<Local>,
    exprs: Vec<&'a dyn Expressable>,
}

impl Expressable for Func<'_> {
    fn to_expression(&self) -> Expression {
        let mut l = vec![Expression::new("func")];

        if let Some(id) = &self.id {
            let mut atom = String::with_capacity(id.len() + 1);
            atom.push('$');
            atom.push_str(id);
            l.push(Expression::new(atom))
        }

        if let Some(export) = &self.export {
            l.push(Expression::new(vec![
                Expression::new("export"),
                Expression::new(export).quote(),
            ]))
        }

        for param in self.params.iter() {
            l.push(param.to_expression());
        }

        for result in self.results.iter() {
            l.push(result.to_expression());
        }

        for local in self.locals.iter() {
            l.push(local.to_expression());
        }

        for expr in self.exprs.iter() {
            l.push(expr.to_expression());
        }

        Expression::new(l)
    }
}

impl<'a> Func<'a> {
    #[doc(hidden)]
    pub fn new(id: Option<String>) -> Self {
        Self {
            id: id,
            export: None,
            params: vec![],
            results: vec![],
            locals: vec![],
            exprs: vec![],
        }
    }

    pub fn with_param(mut self, param: Param) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_result(mut self, result: Result) -> Self {
        self.results.push(result);
        self
    }

    pub fn with_local(mut self, local: Local) -> Self {
        self.locals.push(local);
        self
    }

    pub fn with_instruction(mut self, expr: &'a dyn Expressable) -> Self {
        self.exprs.push(expr);
        self
    }

    pub fn with_export<S>(mut self, export: S) -> Self
    where
        S: Into<String>,
    {
        self.export = Some(export.into());
        self
    }
}

#[macro_export]
macro_rules! func {
    ($id: literal) => {
        $crate::Func::new(Some($id.to_string()))
    };

    () => {{
        $crate::Func::new(None)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::local;
    use crate::result;

    #[test]
    pub fn test_empty_func() {
        assert_eq!(func!().to_expression().to_string(), "(func)");
    }

    #[test]
    pub fn test_empty_func_with_id() {
        assert_eq!(func!("foo").to_expression().to_string(), "(func $foo)");
    }

    #[test]
    pub fn test_empty_func_with_locals() {
        assert_eq!(
            func!()
                .with_local(local!(i32))
                .with_local(local!("foo", i32))
                .to_expression()
                .to_string(),
            "(func (local i32) (local $foo i32))"
        );
    }

    #[test]
    pub fn test_empty_func_with_results() {
        assert_eq!(
            func!()
                .with_result(result!(i32))
                .to_expression()
                .to_string(),
            "(func (result i32))"
        );
    }

    // #[test]
    // pub fn test_empty_func_with_export() {
    //     assert_eq!(func!().to_expression().to_string(), "(func (export ))");
    // }
}
