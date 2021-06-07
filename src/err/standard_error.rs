#[derive(Debug, Clone)]
pub struct StandardError<'a> {
    pub s: &'a str,
}

impl<'a> std::fmt::Display for StandardError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error: {}", self.s)
    }
}

impl<'a> std::error::Error for StandardError<'a> {}
