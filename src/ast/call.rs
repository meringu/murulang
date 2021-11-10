use crate::ast::expression::Expression;
use crate::ast::function::{Function, FunctionSignature};
use crate::ast::variable::{Variable, VariableType};
use crate::ast::variable_name::VariableName;
use crate::err::TypeMismatchError;
use crate::parser::Rule;
use crate::{wasm, wasm_dollar};
use std::collections::HashMap;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::argument))]
pub enum Argument {
    Expression(Expression),
    Literal(Variable),
    VariableName(VariableName),
}

impl Argument {
    pub fn validate(
        &self,
        current_function: &str,
        globals: &std::collections::HashMap<&str, Vec<&Function>>,
        signatures: &mut std::collections::HashMap<&str, FunctionSignature>,
        validated: &mut std::collections::HashSet<&str>,
        local_types: &std::collections::HashMap<&str, VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        match self {
            Argument::Expression(e) => e.validate(
                current_function,
                globals,
                signatures,
                validated,
                local_types,
            ),
            Argument::Literal(t) => Ok(t.get_type()),
            Argument::VariableName(c) => Call {
                variable: VariableName { name: c.name },
                args: vec![],
            }
            .validate(
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
            Argument::Expression(e) => e.to_wasm(return_type, locals_to_arg_index),
            Argument::Literal(t) => t.to_wasm(),
            Argument::VariableName(c) => Call {
                variable: VariableName { name: c.name },
                args: vec![],
            }
            .to_wasm(return_type, locals_to_arg_index),
        }
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::call))]
pub struct Call {
    pub variable: VariableName,
    pub args: Vec<Argument>,
}

impl Call {
    pub fn validate(
        &self,
        current_function: &str,
        globals: &std::collections::HashMap<&str, Vec<&Function>>,
        signatures: &mut std::collections::HashMap<&str, FunctionSignature>,
        validated: &mut std::collections::HashSet<&str>,
        local_types: &std::collections::HashMap<&str, VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        let mut arg_types = vec![];
        for arg in &self.args {
            arg_types.push(arg.validate(
                current_function,
                globals,
                signatures,
                validated,
                local_types,
            )?);
        }

        match local_types.get(self.variable.name) {
            Some(v) => return Ok(*v),
            None => match globals.get(self.variable.name) {
                Some(fns) => {
                    let mut return_types = vec![];
                    for f in fns.into_iter() {
                        return_types.push(f.validate(
                            current_function,
                            globals,
                            signatures,
                            validated,
                            &arg_types,
                        )?);
                    }
                    if return_types.len() > 0 {
                        for &return_type in return_types.iter() {
                            if return_types[0] != return_type {
                                return Err(Box::new(TypeMismatchError {
                                    expected: return_types[0],
                                    got: return_type,
                                }));
                            }
                        }
                        return Ok(return_types[0]);
                    }
                }
                None => {
                    return Err(Box::new(crate::err::FunctionNotFoundError {
                        name: self.variable.name,
                    }))
                }
            },
        };

        match signatures.get(self.variable.name) {
            Some(sig) => Ok(sig.return_type),
            None => Err(Box::new(crate::err::NoFunctionMatchesError {
                name: self.variable.name,
            })),
        }
    }

    pub fn to_wasm(
        &self,
        return_type: VariableType,
        locals_to_arg_index: &HashMap<&str, usize>,
    ) -> wasm::Expression {
        match locals_to_arg_index.get(self.variable.name) {
            Some(i) => wasm!("local.get", i),
            None => {
                let mut call = vec![wasm!("call"), wasm_dollar!(self.variable.name)];
                for arg in self.args.iter() {
                    call.push(arg.to_wasm(return_type, locals_to_arg_index));
                }
                wasm!(call)
            }
        }
    }
}
