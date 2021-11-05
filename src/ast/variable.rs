use crate::ast::util::span_into_str;
use crate::parser::Rule;

use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VariableType {
    Bool,
    Float,
    Int,
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariableType::Int => write!(f, "int"),
            VariableType::Float => write!(f, "float"),
            VariableType::Bool => write!(f, "bool"),
        }
    }
}

impl VariableType {
    pub fn to_wat(&self) -> String {
        match self {
            VariableType::Bool => "i32",
            VariableType::Float => "f32",
            VariableType::Int => "i32",
        }
        .to_owned()
    }
}

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::bool))]
pub struct Bool {
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub val: bool,
}

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::float))]
pub struct Float {
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub val: f64,
}

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::int))]
pub struct Int {
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub val: i64,
}

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::literal))]
pub enum Variable {
    Bool(Bool),
    Float(Float),
    Int(Int),
}

impl Variable {
    pub fn get_type(&self) -> VariableType {
        match self {
            Variable::Bool(_) => VariableType::Bool,
            Variable::Float(_) => VariableType::Float,
            Variable::Int(_) => VariableType::Int,
        }
    }

    pub fn to_wat(&self) -> String {
        match self {
            Variable::Int(l) => format!("(i32.const {})", l.val),
            Variable::Float(l) => format!("(f32.const {})", l.val),
            Variable::Bool(l) => format!("(i32.const {})", l.val as i32),
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variable::Int(v) => write!(f, "{}", v.val),
            Variable::Float(v) => write!(f, "{}", v.val),
            Variable::Bool(v) => write!(f, "{}", v.val),
        }
    }
}
