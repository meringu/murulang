use crate::ast::call::Call;
use crate::ast::function::{Function, FunctionSignature};
use crate::ast::operator::Operator;
use crate::ast::variable::{Variable, VariableType};
use crate::err::{OperatorArgumentError, TypeMismatchError};
use crate::parser::Rule;
use crate::wasm;
use std::collections::{HashMap, HashSet};

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::unary))]
pub enum Unary<'a> {
    Expression(Expression<'a>),
    Literal(Variable),
    Call(Call<'a>),
}

impl<'a> Unary<'a> {
    pub fn validate(
        &self,
        current_function: &'a str,
        globals: &HashMap<&'a str, Vec<&Function<'a>>>,
        signatures: &mut HashMap<&'a str, FunctionSignature>,
        validated: &mut HashSet<&'a str>,
        local_types: &HashMap<&'a str, VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        match self {
            Unary::Expression(e) => e.validate(
                current_function,
                globals,
                signatures,
                validated,
                local_types,
            ),
            Unary::Literal(t) => Ok(t.get_type()),
            Unary::Call(c) => c.validate(
                current_function,
                globals,
                signatures,
                validated,
                local_types,
            ),
        }
    }

    pub fn to_wasm(
        &self,
        return_type: VariableType,
        locals_to_arg_index: &HashMap<&str, usize>,
    ) -> wasm::Expression {
        match self {
            Unary::Expression(e) => e.to_wasm(return_type, locals_to_arg_index),
            Unary::Literal(t) => t.to_wasm(),
            Unary::Call(c) => c.to_wasm(return_type, locals_to_arg_index),
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
    pub fn validate(
        &self,
        current_function: &'a str,
        globals: &HashMap<&'a str, Vec<&Function<'a>>>,
        signatures: &mut HashMap<&'a str, FunctionSignature>,
        validated: &mut HashSet<&'a str>,
        local_types: &HashMap<&'a str, VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        let left_type = self.left.validate(
            current_function,
            globals,
            signatures,
            validated,
            local_types,
        )?;
        let right_type = self.right.validate(
            current_function,
            globals,
            signatures,
            validated,
            local_types,
        )?;
        if left_type != right_type {
            return Err(Box::new(TypeMismatchError {
                expected: left_type,
                got: right_type,
            }));
        }
        Ok(match self.operator {
            Operator::Add(_)
            | Operator::Subtract(_)
            | Operator::Multiply(_)
            | Operator::Divide(_) => match left_type {
                VariableType::Bool => {
                    return Err(Box::new(OperatorArgumentError {
                        operator: self.operator,
                        argument_type: left_type,
                    }))
                }
                _ => left_type,
            },
            Operator::Eq(_) | Operator::Neq(_) => VariableType::Bool,
        })
    }

    pub fn to_wasm(
        &self,
        return_type: VariableType,
        locals_to_arg_index: &HashMap<&str, usize>,
    ) -> wasm::Expression {
        wasm!(
            format!("{}.{}", return_type.to_wasm(), self.operator.to_wasm()),
            self.left.to_wasm(return_type, locals_to_arg_index),
            self.right.to_wasm(return_type, locals_to_arg_index)
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
    pub fn validate(
        &self,
        current_function: &'a str,
        globals: &HashMap<&'a str, Vec<&Function<'a>>>,
        signatures: &mut HashMap<&'a str, FunctionSignature>,
        validated: &mut HashSet<&'a str>,
        local_types: &HashMap<&'a str, VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        let return_type = self.condition.validate(
            current_function,
            globals,
            signatures,
            validated,
            local_types,
        )?;
        if return_type != VariableType::Bool {
            return Err(Box::new(TypeMismatchError {
                expected: VariableType::Bool,
                got: return_type,
            }));
        }
        let truthy_type = self.truthy.validate(
            current_function,
            globals,
            signatures,
            validated,
            local_types,
        )?;
        let falsy_type = self.falsy.validate(
            current_function,
            globals,
            signatures,
            validated,
            local_types,
        )?;
        if truthy_type != falsy_type {
            return Err(Box::new(TypeMismatchError {
                expected: truthy_type,
                got: falsy_type,
            }));
        }
        Ok(truthy_type)
    }

    pub fn to_wasm(
        &self,
        return_type: VariableType,
        locals_to_arg_index: &HashMap<&str, usize>,
    ) -> wasm::Expression {
        wasm!(
            "if",
            wasm!("result", return_type.to_wasm()),
            self.condition.to_wasm(return_type, locals_to_arg_index),
            wasm!(
                "then",
                self.truthy.to_wasm(return_type, locals_to_arg_index)
            ),
            wasm!("else", self.falsy.to_wasm(return_type, locals_to_arg_index))
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
    pub fn validate(
        &self,
        current_function: &'a str,
        globals: &HashMap<&'a str, Vec<&Function<'a>>>,
        signatures: &mut HashMap<&'a str, FunctionSignature>,
        validated: &mut HashSet<&'a str>,
        local_types: &HashMap<&'a str, VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        match self {
            Expression::Unary(u) => u.validate(
                current_function,
                globals,
                signatures,
                validated,
                local_types,
            ),
            Expression::Binary(b) => b.validate(
                current_function,
                globals,
                signatures,
                validated,
                local_types,
            ),
            Expression::Ternary(t) => t.validate(
                current_function,
                globals,
                signatures,
                validated,
                local_types,
            ),
        }
    }

    pub fn to_wasm(
        &self,
        return_type: VariableType,
        locals_to_arg_index: &HashMap<&str, usize>,
    ) -> wasm::Expression {
        match self {
            Expression::Unary(u) => {
                return u.to_wasm(return_type, locals_to_arg_index);
            }
            Expression::Binary(b) => {
                return b.to_wasm(return_type, locals_to_arg_index);
            }
            Expression::Ternary(t) => {
                return t.to_wasm(return_type, locals_to_arg_index);
            }
        }
    }
}
