use crate::ast::VariableType;

#[derive(Debug, Clone)]
pub struct TypeMismatchError {
    pub expected: VariableType,
    pub got: VariableType,
}

impl std::fmt::Display for TypeMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "type mismatch error, expected: {}, got: {}", self.expected, self.got)
    }
}

impl std::error::Error for TypeMismatchError {}
