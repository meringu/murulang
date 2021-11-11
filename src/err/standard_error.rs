#[derive(Debug, Clone)]
pub struct StandardError {
    pub s: String,
}

impl std::fmt::Display for StandardError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: {}", self.s)
    }
}

impl std::error::Error for StandardError {}
