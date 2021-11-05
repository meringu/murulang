use crate::expression::{Expressable, Expression};

pub struct Result {
    ty: String,
}

impl Expressable for Result {
    fn to_expression(&self) -> Expression {
        Expression::new(vec![Expression::new("result"), Expression::new(&self.ty)])
    }
}

impl Result {
    #[doc(hidden)]
    pub fn new(ty: String) -> Self {
        Self { ty: ty }
    }
}

#[macro_export]
macro_rules! result {
    ($ty:ty) => {{
        Result::new(stringify!($ty).to_string())
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_result_i32() {
        assert_eq!(result!(i32).to_expression().to_string(), "(result i32)");
    }

    #[test]
    pub fn test_result_i64() {
        assert_eq!(result!(i64).to_expression().to_string(), "(result i64)");
    }

    #[test]
    pub fn test_result_f32() {
        assert_eq!(result!(f32).to_expression().to_string(), "(result f32)");
    }

    #[test]
    pub fn test_result_f64() {
        assert_eq!(result!(f64).to_expression().to_string(), "(result f64)");
    }
}
