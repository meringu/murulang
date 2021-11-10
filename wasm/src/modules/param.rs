use crate::expression::{Expressable, Expression};

pub struct Param {
    tys: Vec<String>,
    id: Option<String>,
}

impl Param {
    #[doc(hidden)]
    pub fn new(tys: Vec<String>, id: Option<String>) -> Self {
        Param { tys: tys, id: id }
    }
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

        for ty in self.tys.iter() {
            l.push(Expression::new(ty));
        }

        Expression::new(l)
    }
}

#[macro_export]
macro_rules! param {
    ($id: literal, $ty:ty) => {{
        let mut tys = Vec::new();
        tys.push(stringify!($ty).to_string());
        $crate::Param::new(tys, Some($id.to_string()))
    }};

    ($($ty:ty),+) => {{
        let mut tys = Vec::new();
        $(
            tys.push(stringify!($ty).to_string());
        )*
        $crate::Param::new(tys, None)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_param_with_id() {
        assert_eq!(
            param!("foo", i32).to_expression().to_string(),
            "(param $foo i32)"
        )
    }

    #[test]
    pub fn test_param_i32() {
        assert_eq!(param!(i32).to_expression().to_string(), "(param i32)")
    }

    #[test]
    pub fn test_param_all_types() {
        assert_eq!(
            param!(i32, i64, f32, f64).to_expression().to_string(),
            "(param i32 i64 f32 f64)"
        )
    }
}
