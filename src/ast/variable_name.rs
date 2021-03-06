use crate::ast::util::span_into_str;
use crate::parser::Rule;

#[derive(Debug, FromPest, Copy, Clone)]
#[pest_ast(rule(Rule::variable))]
pub struct VariableName<'a> {
    #[pest_ast(outer(with(span_into_str)))]
    pub name: &'a str,
}

impl std::fmt::Display for VariableName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
