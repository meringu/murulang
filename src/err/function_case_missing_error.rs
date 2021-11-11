#[derive(Debug, Clone)]
pub struct FunctionCaseMissingError {
    pub function_name: String,
}

impl std::fmt::Display for FunctionCaseMissingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "function case missing error: {}", self.function_name)
    }
}

impl std::error::Error for FunctionCaseMissingError {}
