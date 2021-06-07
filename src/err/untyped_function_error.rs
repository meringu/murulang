#[derive(Debug, Clone)]
pub struct UntypedFunctionError {
    pub function_name: &'static str,
}

impl std::fmt::Display for UntypedFunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "untyped function error: could not determine type for function {}", self.function_name)
    }
}

impl std::error::Error for UntypedFunctionError {}
