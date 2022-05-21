use crate::ast::call::Call;
use crate::ast::operator::Operator;
use crate::ast::variable::{Variable, VariableType};
use crate::parser::Rule;
use crate::wasm;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::unary))]
pub enum Unary<'a> {
    Expression(Expression<'a>),
    Literal(Variable),
    Call(Call<'a>),
}

impl<'a> Unary<'a> {
    pub fn to_wasm(&self, return_type: VariableType, locals: &Vec<String>) -> wasm::Expression {
        match self {
            Unary::Expression(e) => e.to_wasm(return_type, locals),
            Unary::Literal(t) => t.to_wasm(),
            Unary::Call(c) => c.to_wasm(return_type, locals),
        }
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::binary))]
pub struct Binary<'a> {
    pub left: Unary<'a>,
    pub operator: Operator,
    pub right: Unary<'a>,
}

impl<'a> Binary<'a> {
    pub fn to_wasm(&self, return_type: VariableType, locals: &Vec<String>) -> wasm::Expression {
        wasm!(
            format!("{}.{}", return_type.to_wasm(), self.operator.to_wasm()),
            self.left.to_wasm(return_type, locals),
            self.right.to_wasm(return_type, locals)
        )
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ternary))]
pub struct Ternary<'a> {
    pub condition: Unary<'a>,
    pub truthy: Unary<'a>,
    pub falsy: Unary<'a>,
}

impl<'a> Ternary<'a> {
    pub fn to_wasm(&self, return_type: VariableType, locals: &Vec<String>) -> wasm::Expression {
        wasm!(
            "if",
            wasm!("result", return_type.to_wasm()),
            self.condition.to_wasm(return_type, locals),
            wasm!("then", self.truthy.to_wasm(return_type, locals)),
            wasm!("else", self.falsy.to_wasm(return_type, locals))
        )
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression<'a> {
    Unary(Box<Unary<'a>>),
    Binary(Box<Binary<'a>>),
    Ternary(Box<Ternary<'a>>),
}

impl<'a> Expression<'a> {
    pub fn to_wasm(&self, return_type: VariableType, locals: &Vec<String>) -> wasm::Expression {
        match self {
            Expression::Unary(u) => {
                return u.to_wasm(return_type, locals);
            }
            Expression::Binary(b) => {
                return b.to_wasm(return_type, locals);
            }
            Expression::Ternary(t) => {
                return t.to_wasm(return_type, locals);
            }
        }
    }
}
