#[derive(Debug, Clone)]
pub struct NotImplementedError<'a> {
    pub sub: &'a str,
}

impl<'a> std::fmt::Display for NotImplementedError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "not implemented error: {}", self.sub)
    }
}

impl<'a> std::error::Error for NotImplementedError<'a> {}
