use crate::expression::{Expressable, Expression};

pub struct Call<'a> {
    name: String,
    args: Vec<&'a dyn Expressable>,
}

impl Expressable for Call<'_> {
    fn to_expression(&self) -> Expression {
        let mut l = vec![Expression::new("call")];

        let mut atom = String::with_capacity(self.name.len() + 1);
        atom.push('$');
        atom.push_str(&self.name);
        l.push(Expression::new(atom));

        for args in self.args.iter() {
            l.push(args.to_expression());
        }

        Expression::new(l)
    }
}

impl<'a> Call<'a> {
    #[doc(hidden)]
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            args: vec![],
        }
    }

    pub fn with_arg(mut self, arg: &'a dyn Expressable) -> Self {
        self.args.push(arg);
        self
    }
}

#[macro_export]
macro_rules! call {
    ($name:literal) => {
        $crate::Call::new($name.to_string())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::I32Const;

    #[test]
    pub fn test_call() {
        assert_eq!(call!("foo").to_expression().to_string(), "(call $foo)");
    }

    #[test]
    pub fn test_call_with_arg() {
        assert_eq!(
            call!("foo")
                .with_arg(&I32Const::new(0))
                .to_expression()
                .to_string(),
            "(call $foo (i32.const 0))"
        );
    }
}
