use crate::ast::call::Call;
use crate::ast::function::{Function, FunctionSignature};
use crate::ast::operator::Operator;
use crate::ast::variable::{Variable, VariableType};
use crate::parser::Rule;
use crate::err::{OperatorArgumentError, TypeMismatchError};
use crate::wat;
use std::collections::{HashSet, HashMap};

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::unary))]
pub enum Unary {
    Expression(Expression),
    Literal(Variable),
    Call(Call)
}

impl Unary {
    pub fn validate(
        &self,
        current_function: &str,
        globals: &HashMap::<&str, Vec<&Function>>,
        signatures: &mut HashMap::<&str, FunctionSignature>,
        validated: &mut HashSet::<&str>,
        local_types: &HashMap::<&str, VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        match self {
            Unary::Expression(e) => e.validate(current_function, globals, signatures, validated, local_types),
            Unary::Literal(t) => Ok(t.get_type()),
            Unary::Call(c) => c.validate(current_function, globals, signatures, validated, local_types),
        }
    }

    pub fn to_wat(&self, return_type: VariableType, locals_to_arg_index: &HashMap::<&str, usize>) -> String {
        match self {
            Unary::Expression(e) => { return e.to_wat(return_type, locals_to_arg_index); },
            Unary::Literal(t) => { return t.to_wat(); },
            Unary::Call(c) => { return c.to_wat(return_type, locals_to_arg_index) },
        }
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::binary))]
pub struct Binary {
    pub left: Unary,
    pub operator: Operator,
    pub right: Unary,
}

impl Binary {
    pub fn validate(
        &self,
        current_function: &str,
        globals: &HashMap::<&str, Vec<&Function>>,
        signatures: &mut HashMap::<&str, FunctionSignature>,
        validated: &mut HashSet::<&str>,
        local_types: &HashMap::<&str, VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        let left_type = self.left.validate(current_function, globals, signatures, validated, local_types)?;
        let right_type = self.right.validate(current_function, globals, signatures, validated, local_types)?;
        if left_type != right_type {
            return Err(Box::new(TypeMismatchError{expected: left_type, got: right_type}));
        }
        Ok(match self.operator {
            Operator::Add(_) | Operator::Subtract(_) | Operator::Multiply(_) | Operator::Divide(_) => match left_type {
                VariableType::Bool => {
                    return Err(Box::new(OperatorArgumentError{operator: self.operator, argument_type: left_type}))
                },
                _ => left_type,
            },
            Operator::Eq(_) | Operator::Neq(_) => VariableType::Bool,
        })
    }

    pub fn to_wat(&self, return_type: VariableType, locals_to_arg_index: &HashMap::<&str, usize>) -> String {
        format!("({}.{}
    {}
    {}
)
",
            return_type.to_wat(),
            self.operator.to_wat(),
            wat::indent(self.left.to_wat(return_type, locals_to_arg_index), 4),
            wat::indent(self.right.to_wat(return_type, locals_to_arg_index), 4)
        )
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::ternary))]
pub struct Ternary {
    pub condition: Unary,
    pub truthy: Unary,
    pub falsy: Unary,
}

impl Ternary {
    pub fn validate(
        &self,
        current_function: &str,
        globals: &HashMap::<&str, Vec<&Function>>,
        signatures: &mut HashMap::<&str, FunctionSignature>,
        validated: &mut HashSet::<&str>,
        local_types: &HashMap::<&str, VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        let return_type = self.condition.validate(current_function, globals, signatures, validated, local_types)?;
        if return_type != VariableType::Bool {
            return Err(Box::new(TypeMismatchError{expected: VariableType::Bool, got: return_type}));
        }
        let truthy_type = self.truthy.validate(current_function, globals, signatures, validated, local_types)?;
        let falsy_type = self.falsy.validate(current_function, globals, signatures, validated, local_types)?;
        if truthy_type != falsy_type {
            return Err(Box::new(TypeMismatchError{expected: truthy_type, got: falsy_type}));
        }
        Ok(truthy_type)
    }

    pub fn to_wat(&self, return_type: VariableType, locals_to_arg_index: &HashMap::<&str, usize>) -> String {
        wat::control_if(
            Some(return_type.to_wat()),
            self.condition.to_wat(return_type, locals_to_arg_index),
            self.truthy.to_wat(return_type, locals_to_arg_index),
            Some(self.falsy.to_wat(return_type, locals_to_arg_index)),
        )
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression {
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Ternary(Box<Ternary>),
}

impl Expression {
    pub fn validate(
        &self,
        current_function: &str,
        globals: &HashMap::<&str, Vec<&Function>>,
        signatures: &mut HashMap::<&str, FunctionSignature>,
        validated: &mut HashSet::<&str>,
        local_types: &HashMap::<&str, VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        match self {
            Expression::Unary(u) => u.validate(current_function, globals, signatures, validated, local_types),
            Expression::Binary(b) => b.validate(current_function, globals, signatures, validated, local_types),
            Expression::Ternary(t) => t.validate(current_function, globals, signatures, validated, local_types),
        }
    }

    pub fn to_wat(&self, return_type: VariableType, locals_to_arg_index: &HashMap::<&str, usize>) -> String {
        match self {
            Expression::Unary(u) => { return u.to_wat(return_type, locals_to_arg_index); },
            Expression::Binary(b) => { return b.to_wat(return_type, locals_to_arg_index); },
            Expression::Ternary(t) => { return t.to_wat(return_type, locals_to_arg_index); },
        }
    }
}
