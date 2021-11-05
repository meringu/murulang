use crate::ast::{Operator, VariableType};

#[derive(Debug, Clone)]
pub struct OperatorArgumentError {
    pub operator: Operator,
    pub argument_type: VariableType,
}

impl std::fmt::Display for OperatorArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "no implementation of {} for {}",
            self.operator, self.argument_type
        )
    }
}

impl std::error::Error for OperatorArgumentError {}
