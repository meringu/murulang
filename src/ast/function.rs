use crate::ast::expression::Expression;
use crate::ast::variable::{Variable, VariableType};
use crate::ast::variable_name::VariableName;
use crate::err::{ArgumentError, TypeMismatchError, UntypedFunctionError};
use crate::parser::Rule;
use pest::Span;
use std::collections::{HashMap, HashSet};

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::parameter))]
pub enum FunctionParameter {
    Variable(VariableName),
    Literal(Variable),
}

fn span_into_variable_type(span: Span) -> VariableType {
    match span.as_str() {
        "bool" => VariableType::Bool,
        "float" => VariableType::Float,
        "int" => VariableType::Int,
        _ => unreachable!(),
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::function_signature))]
pub struct AstFunctionSignature {
    pub name: VariableName,
    pub types: Vec<VarType>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::var_type))]
pub struct VarType {
    #[pest_ast(inner(with(span_into_variable_type)))]
    pub var_type: VariableType,
}

#[derive(Debug)]
pub struct FunctionSignature {
    pub arg_types: Vec<VariableType>,
    pub return_type: VariableType,
}

impl std::fmt::Display for FunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for arg_type in &self.arg_types {
            write!(f, "{} -> ", arg_type)?;
        }
        write!(f, "{}", self.return_type)
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::function))]
pub struct Function {
    pub name: VariableName,
    pub parameters: Vec<FunctionParameter>,
    pub expr: Expression,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::line))]
pub enum Line {
    FunctionSignature(AstFunctionSignature),
    Function(Function),
}

impl Function {
    pub fn validate(
        &self,
        current_function: &str,
        globals: &HashMap<&str, Vec<&Function>>,
        signatures: &mut HashMap<&str, FunctionSignature>,
        validated: &mut HashSet<&str>,
        arg_types: &Vec<VariableType>,
    ) -> Result<VariableType, Box<dyn std::error::Error>> {
        if current_function == self.name.name && !signatures.contains_key(self.name.name) {
            return Err(Box::new(UntypedFunctionError {
                function_name: self.name.name,
            }));
        }

        if arg_types.len() != self.parameters.len() {
            return Err(Box::new(crate::err::ArgumentError {
                function_name: self.name.name,
                expected: self.parameters.len(),
                actual: arg_types.len(),
            }));
        }

        for i in 0..self.parameters.len() {
            match &self.parameters[i] {
                FunctionParameter::Variable(_) => {}
                FunctionParameter::Literal(l) => {
                    if l.get_type() != arg_types[i] {
                        return Err(Box::new(TypeMismatchError {
                            expected: l.get_type(),
                            got: arg_types[i],
                        }));
                    }
                }
            };
        }

        let mut local_types = HashMap::<&str, VariableType>::new();
        for i in 0..self.parameters.len() {
            match &self.parameters[i] {
                FunctionParameter::Variable(v) => {
                    local_types.insert(v.name, arg_types[i]);
                }
                FunctionParameter::Literal(_) => {}
            };
        }

        if validated.contains(self.name.name) {
            let signature = signatures.get(self.name.name).unwrap();
            if arg_types.len() != signature.arg_types.len() {
                return Err(Box::new(ArgumentError {
                    function_name: self.name.name,
                    expected: signature.arg_types.len(),
                    actual: arg_types.len(),
                }));
            }
            for i in 0..arg_types.len() {
                if arg_types[i] != signature.arg_types[i] {
                    return Err(Box::new(TypeMismatchError {
                        expected: signature.arg_types[i],
                        got: arg_types[i],
                    }));
                }
            }
            return Ok(signature.return_type);
        }

        validated.insert(self.name.name);
        Ok(match signatures.get(self.name.name) {
            Some(sig) => sig.return_type,
            None => {
                let return_type = self.expr.validate(
                    self.name.name,
                    globals,
                    signatures,
                    validated,
                    &local_types,
                )?;
                let signature = FunctionSignature {
                    arg_types: arg_types.to_vec(),
                    return_type: return_type,
                };
                signatures.insert(self.name.name, signature);
                return_type
            }
        })
    }

    pub fn to_wat(&self, return_type: VariableType) -> String {
        let mut locals_to_arg_index = HashMap::<&str, usize>::new();
        for i in 0..self.parameters.len() {
            match &self.parameters[i] {
                FunctionParameter::Variable(v) => {
                    locals_to_arg_index.insert(v.name, i);
                }
                FunctionParameter::Literal(_) => {}
            };
        }

        format!("{}", self.expr.to_wat(return_type, &locals_to_arg_index))
    }

    pub fn wat_matches_condition(&self) -> Option<String> {
        let mut conditions = Vec::<String>::new();

        for (i, param) in self.parameters.iter().enumerate() {
            match param {
                FunctionParameter::Variable(_) => {}
                FunctionParameter::Literal(l) => {
                    conditions.push(format!(
                        r#"({}.eq
    (get_local {})
    ({}.const {})
)"#,
                        l.get_type().to_wat(),
                        i,
                        l.get_type().to_wat(),
                        l,
                    ));
                }
            }
        }

        if conditions.len() == 0 {
            return None;
        }
        Some(combine_wat_conditions(&mut conditions))
    }
}

pub fn combine_wat_conditions(conditions: &mut Vec<String>) -> String {
    let left = conditions.pop().unwrap();
    let right = match conditions.len() {
        0 => return left,
        1 => conditions.pop().unwrap(),
        _ => combine_wat_conditions(conditions),
    };

    format!(
        r#"(i32.and
    {}
    {}
)"#,
        left.split("\n").collect::<Vec<&str>>().join("\n    "),
        right.split("\n").collect::<Vec<&str>>().join("\n    "),
    )
}
