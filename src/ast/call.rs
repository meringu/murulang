use crate::ast::expression::Expression;
use crate::ast::variable::{Variable, VariableType};
use crate::ast::variable_name::VariableName;
use crate::parser::Rule;
use crate::{wasm, wasm_dollar};

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::argument))]
pub enum Argument<'a> {
    Expression(Expression<'a>),
    Literal(Variable),
    VariableName(VariableName<'a>),
}

impl<'a> Argument<'a> {
    pub fn to_wasm(&self, return_type: VariableType, locals: &Vec<String>) -> wasm::Expression {
        match self {
            Argument::Expression(e) => e.to_wasm(return_type, locals),
            Argument::Literal(t) => t.to_wasm(),
            Argument::VariableName(c) => Call {
                variable: VariableName { name: c.name },
                args: vec![],
            }
            .to_wasm(return_type, locals),
        }
    }
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::call))]
pub struct Call<'a> {
    pub variable: VariableName<'a>,
    pub args: Vec<Argument<'a>>,
}

impl<'a> Call<'a> {
    pub fn to_wasm(&self, return_type: VariableType, locals: &Vec<String>) -> wasm::Expression {
        for local in locals.iter() {
            if local == self.variable.name {
                return wasm!("local.get", wasm_dollar!(local));
            }
        }

        let mut call = vec![wasm!("call"), wasm_dollar!(self.variable.name)];
        for arg in self.args.iter() {
            call.push(arg.to_wasm(return_type, locals));
        }
        wasm!(call)
    }
}
