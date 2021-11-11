#[derive(Debug, Clone)]
pub struct NoFunctionMatchesError {
    pub name: String,
}

impl std::fmt::Display for NoFunctionMatchesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "no match for function error: {}", self.name)
    }
}

impl std::error::Error for NoFunctionMatchesError {}
