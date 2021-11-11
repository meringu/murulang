#[derive(Debug, Clone)]
pub struct FunctionAlreadyDefinedError {
    pub function_name: String,
}

impl std::fmt::Display for FunctionAlreadyDefinedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "function already defined error: {}", self.function_name)
    }
}

impl std::error::Error for FunctionAlreadyDefinedError {}
