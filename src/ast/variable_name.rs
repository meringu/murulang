use crate::ast::util::{span_into_str, string_to_static_str};
use crate::parser::Rule;

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::variable))]
pub struct VariableName {
    #[pest_ast(outer(with(span_into_str), with(string_to_static_str)))]
    pub name: &'static str,
}

impl std::fmt::Display for VariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
