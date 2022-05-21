use crate::ast::expression::Expression;
use crate::ast::variable::VariableType;
use crate::ast::variable_name::VariableName;
use crate::parser::Rule;
use crate::{wasm, wasm_dollar};
use pest::Span;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::parameter))]
pub struct FunctionParameter<'a> {
    name: VariableName<'a>,
    ty: VarType,
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
#[pest_ast(rule(Rule::var_type))]
pub struct VarType {
    #[pest_ast(inner(with(span_into_variable_type)))]
    pub var_type: VariableType,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::function))]
pub struct Function<'a> {
    pub name: VariableName<'a>,
    pub parameters: Vec<FunctionParameter<'a>>,
    pub return_type: VarType,
    pub exprs: Vec<Expression<'a>>,
}

impl<'a> Function<'a> {
    pub fn to_wasm(&self) -> wasm::Expression {
        let mut func = wasm!(wasm!("func"), wasm_dollar!(self.name.name));

        let mut locals = vec![];
        if self.parameters.len() > 0 {
            let mut param = wasm!("param");
            for parameter in self.parameters.iter() {
                locals.push(parameter.name.name.to_string());
                param = param.extend(parameter.ty.var_type.to_wasm());
            }
            func = func.extend(param);
        }

        func = func.extend(wasm!("result", self.return_type.var_type.to_wasm()));

        for expr in self.exprs.iter() {
            func = func.extend(expr.to_wasm(self.return_type.var_type, &locals));
        }

        func
    }
}
