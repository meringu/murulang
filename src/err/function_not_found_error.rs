#[derive(Debug, Clone)]
pub struct FunctionNotFoundError<'a> {
    pub name: &'a str,
}

impl<'a> std::fmt::Display for FunctionNotFoundError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "function not found error: {}", self.name)
    }
}

impl<'a> std::error::Error for FunctionNotFoundError<'a> {}
