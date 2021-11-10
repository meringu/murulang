use crate::expression::{Expressable, Expression};

pub struct Local {
    tys: Vec<String>,
    id: Option<String>,
}

impl Local {
    #[doc(hidden)]
    pub fn new(tys: Vec<String>, id: Option<String>) -> Self {
        Local { tys: tys, id: id }
    }
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

        for ty in self.tys.iter() {
            l.push(Expression::new(ty));
        }

        Expression::new(l)
    }
}

#[macro_export]
macro_rules! local {
    ($id: literal, $ty:ty) => {{
        let mut tys = Vec::new();
        tys.push(stringify!($ty).to_string());
        $crate::Local::new(tys, Some($id.to_string()),)
    }};

    ($($ty:ty),+) => {{
        let mut tys = Vec::new();
        $(
            tys.push(stringify!($ty).to_string());
        )*
        $crate::Local::new(tys, None)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_local_with_id() {
        assert_eq!(
            local!("foo", i32).to_expression().to_string(),
            "(local $foo i32)"
        )
    }

    #[test]
    pub fn test_local_i32() {
        assert_eq!(local!(i32).to_expression().to_string(), "(local i32)")
    }

    #[test]
    pub fn test_local_all_types() {
        assert_eq!(
            local!(i32, i64, f32, f64).to_expression().to_string(),
            "(local i32 i64 f32 f64)"
        )
    }
}
