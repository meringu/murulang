use crate::expression::{Expressable, Expression};

pub struct Param {
    ty: String,
    id: Option<String>,
}

impl Expressable for Param {
    fn to_expression(&self) -> Expression {
        let mut l = vec![Expression::new("param")];

        if let Some(id) = &self.id {
            let mut atom = String::with_capacity(id.len() + 1);
            atom.push('$');
            atom.push_str(id);
            l.push(Expression::new(atom))
        }

        l.push(Expression::new(&self.ty));

        Expression::new(l)
    }
}

impl Param {
    pub fn from<S: Into<String>>(ty: S) -> Self {
        Self {
            ty: ty.into(),
            id: None,
        }
    }

    pub fn i32() -> Self {
        Self::from("i32")
    }

    pub fn i64() -> Self {
        Self::from("i64")
    }

    pub fn f32() -> Self {
        Self::from("f32")
    }

    pub fn f64() -> Self {
        Self::from("f64")
    }

    pub fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_param_i32() {
        assert_eq!(Param::i32().to_expression().to_string(), "(param i32)");
    }

    #[test]
    pub fn test_param_i32_with_id() {
        assert_eq!(
            Param::i32().with_id("foo").to_expression().to_string(),
            "(param $foo i32)"
        );
    }

    #[test]
    pub fn test_param_i64() {
        assert_eq!(Param::i64().to_expression().to_string(), "(param i64)");
    }

    #[test]
    pub fn test_param_f32() {
        assert_eq!(Param::f32().to_expression().to_string(), "(param f32)");
    }

    #[test]
    pub fn test_param_f64() {
        assert_eq!(Param::f64().to_expression().to_string(), "(param f64)");
    }
}
