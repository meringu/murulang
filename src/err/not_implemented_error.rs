#[derive(Debug, Clone)]
pub struct NotImplementedError {
    pub sub: String,
}

impl std::fmt::Display for NotImplementedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "not implemented error: {}", self.sub)
    }
}

impl std::error::Error for NotImplementedError {}
