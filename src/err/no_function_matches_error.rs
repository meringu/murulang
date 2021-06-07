#[derive(Debug, Clone)]
pub struct NoFunctionMatchesError<'a> {
    pub name: &'a str,
}

impl<'a> std::fmt::Display for NoFunctionMatchesError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "no match for function error: {}", self.name)
    }
}

impl<'a> std::error::Error for NoFunctionMatchesError<'a> {}
