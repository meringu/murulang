use crate::expression::{Expressable, Expression};

pub struct Local {
    ty: String,
    id: Option<String>,
}

impl Expressable for Local {
    fn to_expression(&self) -> Expression {
        let mut l = vec![Expression::new("local")];

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

impl Local {
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
        Self::from("i32")
    }

    pub fn f32() -> Self {
        Self::from("i32")
    }

    pub fn f64() -> Self {
        Self::from("i32")
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
    pub fn test_local_i32() {
        assert_eq!(Local::i32().to_expression().to_string(), "(local i32)");
    }

    #[test]
    pub fn test_local_i32_with_id() {
        assert_eq!(
            Local::i32().with_id("foo").to_expression().to_string(),
            "(local $foo i32)"
        );
    }
}
