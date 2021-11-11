#[derive(Debug, Clone)]
pub struct ArgumentError {
    pub function_name: String,
    pub expected: usize,
    pub actual: usize,
}

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "argument error: {} expected: {}, got: {}",
            self.function_name, self.expected, self.actual
        )
    }
}

impl std::error::Error for ArgumentError {}
