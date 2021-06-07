#[derive(Debug, Clone)]
pub struct FunctionCaseUnreachableError {
    pub function_name: &'static str,
}

impl std::fmt::Display for FunctionCaseUnreachableError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "function case unreachable error: {}", self.function_name)
    }
}

impl std::error::Error for FunctionCaseUnreachableError {}
