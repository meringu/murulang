use crate::expression::{Expressable, Expression};

macro_rules! make_variable {
    ($name: ident, $instruction: literal) => {
        pub enum $name<'a> {
            Id(u32),
            Name(&'a str),
        }

        impl<'a> $name<'a> {
            pub fn new<A>(a: A) -> Self
            where
                A: Into<$name<'a>>,
            {
                a.into()
            }
        }

        impl<'a> From<u32> for $name<'a> {
            fn from(id: u32) -> $name<'a> {
                $name::Id(id)
            }
        }

        impl<'a> From<&'a str> for $name<'a> {
            fn from(name: &'a str) -> $name<'a> {
                $name::Name(name)
            }
        }

        impl Expressable for $name<'_> {
            fn to_expression(&self) -> Expression {
                Expression::List(vec![
                    Expression::Atom($instruction.to_string()),
                    Expression::Atom(match self {
                        $name::Id(id) => id.to_string(),
                        $name::Name(name) => {
                            let mut refr = String::with_capacity(name.len() + 1);
                            refr.push('$');
                            refr.push_str(name);
                            refr
                        }
                    }),
                ])
            }
        }
    };
}

make_variable!(LocalGet, "local.get");
make_variable!(LocalSet, "local.set");
make_variable!(LocalTee, "local.tee");
make_variable!(GlobalGet, "global.get");
make_variable!(GlobalSet, "global.set");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_get() {
        assert_eq!(
            LocalGet::new(0).to_expression().to_string(),
            "(local.get 0)"
        );
    }

    #[test]
    fn test_local_get_named() {
        assert_eq!(
            LocalGet::new("foo").to_expression().to_string(),
            "(local.get $foo)"
        );
    }

    #[test]
    fn test_local_set() {
        assert_eq!(
            LocalSet::new(0).to_expression().to_string(),
            "(local.set 0)"
        );
    }

    #[test]
    fn test_local_set_named() {
        assert_eq!(
            LocalSet::new("foo").to_expression().to_string(),
            "(local.set $foo)"
        );
    }

    #[test]
    fn test_local_tee() {
        assert_eq!(
            LocalTee::new(0).to_expression().to_string(),
            "(local.tee 0)"
        );
    }

    #[test]
    fn test_local_tee_named() {
        assert_eq!(
            LocalTee::new("foo").to_expression().to_string(),
            "(local.tee $foo)"
        );
    }

    #[test]
    fn test_global_get() {
        assert_eq!(
            GlobalGet::new(0).to_expression().to_string(),
            "(global.get 0)"
        );
    }

    #[test]
    fn test_global_get_named() {
        assert_eq!(
            GlobalGet::new("foo").to_expression().to_string(),
            "(global.get $foo)"
        );
    }

    #[test]
    fn test_global_set() {
        assert_eq!(
            GlobalSet::new(0).to_expression().to_string(),
            "(global.set 0)"
        );
    }

    #[test]
    fn test_global_set_named() {
        assert_eq!(
            GlobalSet::new("foo").to_expression().to_string(),
            "(global.set $foo)"
        );
    }
}
