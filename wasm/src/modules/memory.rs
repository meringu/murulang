use crate::expression::{Expressable, Expression};

pub struct Memory {
    id: Option<String>,
    min: u32,
    max: Option<u32>,
}

impl Memory {
    #[doc(hidden)]
    pub fn new(min: u32, max: Option<u32>) -> Self {
        Memory {
            id: None,
            min: min,
            max: max,
        }
    }

    pub fn with_id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl Expressable for Memory {
    fn to_expression(&self) -> Expression {
        let mut l = vec![Expression::new("memory")];

        if let Some(id) = &self.id {
            let mut atom = String::with_capacity(id.len() + 1);
            atom.push('$');
            atom.push_str(id);
            l.push(Expression::new(atom))
        }

        l.push(Expression::new(self.min.to_string()));

        if let Some(max) = self.max {
            l.push(Expression::new(max.to_string()))
        }

        Expression::new(l)
    }
}

#[macro_export]
macro_rules! memory {
    ($min:literal) => {
        $crate::Memory::new($min, None)
    };

    ($min:literal, $max:literal) => {
        $crate::Memory::new($min, Some($max))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_anon_memory() {
        assert_eq!(memory!(0).to_expression().to_string(), "(memory 0)")
    }

    #[test]
    pub fn test_anon_lim_memory() {
        assert_eq!(memory!(0, 1).to_expression().to_string(), "(memory 0 1)")
    }

    #[test]
    pub fn test_id_memory() {
        assert_eq!(
            memory!(0).with_id("foo").to_expression().to_string(),
            "(memory $foo 0)"
        )
    }

    #[test]
    pub fn test_id_lim_memory() {
        assert_eq!(
            memory!(0, 1).with_id("foo").to_expression().to_string(),
            "(memory $foo 0 1)"
        )
    }
}
