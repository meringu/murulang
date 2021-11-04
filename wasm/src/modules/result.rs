use crate::expression::{Expressable, Expression};

pub struct Result {
    ty: String,
}

impl Expressable for Result {
    fn to_expression(&self) -> Expression {
        let mut l = vec![Expression::new("result")];

        l.push(Expression::new(&self.ty));

        Expression::new(l)
    }
}

impl Result {
    pub fn from<S: Into<String>>(ty: S) -> Self {
        Self { ty: ty.into() }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_result_i32() {
        assert_eq!(Result::i32().to_expression().to_string(), "(result i32)");
    }

    #[test]
    pub fn test_result_i64() {
        assert_eq!(Result::i64().to_expression().to_string(), "(result i64)");
    }

    #[test]
    pub fn test_result_f32() {
        assert_eq!(Result::f32().to_expression().to_string(), "(result f32)");
    }

    #[test]
    pub fn test_result_f64() {
        assert_eq!(Result::f64().to_expression().to_string(), "(result f64)");
    }
}
