use super::util::span_into_phantomdata;
use crate::parser::Rule;
use crate::{wasm, wasm::Expression};
use std::marker::PhantomData;

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::add))]
pub struct Add {
    #[pest_ast(outer(with(span_into_phantomdata)))]
    n: PhantomData<()>,
}

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::subtract))]
pub struct Subtract {
    #[pest_ast(outer(with(span_into_phantomdata)))]
    n: PhantomData<()>,
}

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::multiply))]
pub struct Multiply {
    #[pest_ast(outer(with(span_into_phantomdata)))]
    n: PhantomData<()>,
}

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::divide))]
pub struct Divide {
    #[pest_ast(outer(with(span_into_phantomdata)))]
    n: PhantomData<()>,
}

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::eq))]
pub struct Eq {
    #[pest_ast(outer(with(span_into_phantomdata)))]
    n: PhantomData<()>,
}

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::neq))]
pub struct Neq {
    #[pest_ast(outer(with(span_into_phantomdata)))]
    n: PhantomData<()>,
}

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::operator))]
pub enum Operator {
    Add(Add),
    Subtract(Subtract),
    Multiply(Multiply),
    Divide(Divide),
    Eq(Eq),
    Neq(Neq),
}

impl Operator {
    pub fn to_wasm(&self) -> Expression {
        match self {
            Operator::Add(_) => wasm!("add"),
            Operator::Subtract(_) => wasm!("sub"),
            Operator::Multiply(_) => wasm!("mul"),
            Operator::Divide(_) => wasm!("div"),
            Operator::Eq(_) => wasm!("eq"),
            Operator::Neq(_) => wasm!("ne"),
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Add(_) => write!(f, "add"),
            Operator::Subtract(_) => write!(f, "sub"),
            Operator::Multiply(_) => write!(f, "mul"),
            Operator::Divide(_) => write!(f, "div"),
            Operator::Eq(_) => write!(f, "eq"),
            Operator::Neq(_) => write!(f, "ne"),
        }
    }
}
