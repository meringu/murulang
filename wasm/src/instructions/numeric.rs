// Defines all the numeric instrucitons
//
// See: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric
//

use crate::expression::{Expressable, Expression};
use crate::instructions::helpers::ident_to_instruction;

macro_rules! make_const {
    ($name: ident, $ty: ty) => {
        pub struct $name {
            n: $ty,
        }

        impl $name {
            pub fn new(n: $ty) -> Self {
                $name { n: n }
            }
        }

        impl Expressable for $name {
            fn to_expression(&self) -> Expression {
                Expression::List(vec![
                    Expression::Atom(format!("{}.const", stringify!($ty))),
                    Expression::Atom(self.n.to_string()),
                ])
            }
        }
    };
}

macro_rules! make_unop {
    ($($name: ident),+) => {
        $(
            pub struct $name<'a> {
                expr: Option<&'a dyn Expressable>,
            }

            impl<'a> $name<'a> {
                pub fn new() -> Self {
                    Self { expr: None }
                }

                pub fn with(&mut self, expr: &'a dyn Expressable) -> &mut Self {
                    self.expr = Some(expr);
                    self
                }
            }

            impl Expressable for $name<'_> {
                fn to_expression(&self) -> Expression {
                    let mut l = vec![Expression::Atom(ident_to_instruction!($name))];

                    if let Some(expr) = self.expr {
                        l.push(expr.to_expression());
                    }

                    Expression::List(l)
                }
            }
        )*
    };
}

macro_rules! make_binop {
    ($($name: ident),+) => {
        $(
            pub struct $name<'a> {
                left: Option<&'a dyn Expressable>,
                right: Option<&'a dyn Expressable>,
            }

            impl<'a> $name<'a> {
                pub fn new() -> Self {
                    Self {
                        left: None,
                        right: None,
                    }
                }

                pub fn with_left(&mut self, expr: &'a dyn Expressable) -> &mut Self {
                    self.left = Some(expr);
                    self
                }

                pub fn with_right(&mut self, expr: &'a dyn Expressable) -> &mut Self {
                    self.right = Some(expr);
                    self
                }
            }

            impl Expressable for $name<'_> {
                fn to_expression(&self) -> Expression {
                    let mut l = vec![Expression::Atom(ident_to_instruction!($name))];

                    if let Some(expr) = self.left {
                        l.push(expr.to_expression());
                    }

                    if let Some(expr) = self.right {
                        l.push(expr.to_expression());
                    }

                    Expression::List(l)
                }
            }
        )*
    };
}

make_const!(F32Const, f32);
make_const!(F64Const, f64);
make_const!(I32Const, i32);
make_const!(I64Const, i64);

make_unop!(
    F32Abs, F32Ceil, F32Floor, F32Nearest, F32Neg, F32Sqrt, F32Trunc, F64Abs, F64Ceil, F64Floor,
    F64Nearest, F64Neg, F64Sqrt, F64Trunc, I32Clz, I32Ctz, I32Popcnt, I64Clz, I64Ctz, I64Popcnt
);

make_binop!(
    I32Add, I32Sub, I32Mul, I32DivU, I32DivS, I32RemU, I32RemS, I32And, I32Or, I32Xor, I32Shl,
    I32ShrU, I32ShrS, I32Rotl, I32Rotr, I64Add, I64Sub, I64Mul, I64DivU, I64DivS, I64RemU, I64RemS,
    I64And, I64Or, I64Xor, I64Shl, I64ShrU, I64ShrS, I64Rotl, I64Rotr
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_const() {
        assert_eq!(
            I32Const::new(0).to_expression().to_string(),
            "(i32.const 0)"
        );
    }

    #[test]
    fn test_f32_const() {
        assert_eq!(
            F32Const::new(0.1).to_expression().to_string(),
            "(f32.const 0.1)"
        );
    }

    #[test]
    fn test_i32_clz() {
        assert_eq!(I32Clz::new().to_expression().to_string(), "(i32.clz)");
    }

    #[test]
    fn test_i32_clz_with_const() {
        assert_eq!(
            I32Clz::new()
                .with(&I32Const::new(0))
                .to_expression()
                .to_string(),
            "(i32.clz (i32.const 0))"
        );
    }

    #[test]
    fn test_i32_add() {
        assert_eq!(I32Add::new().to_expression().to_string(), "(i32.add)");
    }

    #[test]
    fn test_i32_add_with_const() {
        assert_eq!(
            I32Add::new()
                .with_left(&I32Const::new(0))
                .with_right(&I32Const::new(1))
                .to_expression()
                .to_string(),
            "(i32.add (i32.const 0) (i32.const 1))"
        );
    }

    #[test]
    fn test_i32_div_u() {
        assert_eq!(I32DivU::new().to_expression().to_string(), "(i32.div_u)");
    }

    #[test]
    fn test_i32_div_u_with_const() {
        assert_eq!(
            I32DivU::new()
                .with_left(&I32Const::new(0))
                .with_right(&I32Const::new(1))
                .to_expression()
                .to_string(),
            "(i32.div_u (i32.const 0) (i32.const 1))"
        );
    }
}
