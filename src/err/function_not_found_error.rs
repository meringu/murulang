#[derive(Debug, Clone)]
pub struct FunctionNotFoundError {
    pub name: String,
}

impl std::fmt::Display for FunctionNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "function not found error: {}", self.name)
    }
}

impl std::error::Error for FunctionNotFoundError {}
