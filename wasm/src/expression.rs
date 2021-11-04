use core::fmt;
use std::error;
use wabt::wat2wasm;

pub trait Expressable {
    fn to_expression(&self) -> Expression;
}

pub enum Expression {
    Atom(String),
    List(Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_pretty(0))
    }
}

impl From<String> for Expression {
    fn from(s: String) -> Expression {
        Expression::Atom(s)
    }
}

impl From<&str> for Expression {
    fn from(s: &str) -> Expression {
        Expression::Atom(s.to_string())
    }
}

impl From<&String> for Expression {
    fn from(s: &String) -> Expression {
        Expression::Atom(s.to_string())
    }
}

impl From<Vec<Expression>> for Expression {
    fn from(l: Vec<Expression>) -> Expression {
        Expression::List(l)
    }
}

impl Expression {
    pub fn new<A>(a: A) -> Self
    where
        A: Into<Expression>,
    {
        a.into()
    }

    pub fn to_bin(self) -> Result<Vec<u8>, Box<dyn error::Error>> {
        Ok(wat2wasm(self.to_string())?)
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let indent = " ".repeat(width);
        let line_break = if width > 0 { "\n" } else { " " };
        let final_break = if width > 0 { "\n" } else { "" };

        format!(
            "{}",
            match self {
                Expression::Atom(s) => s.to_string(),
                Expression::List(v) => {
                    let has_depth = v
                        .iter()
                        .map(|sexp| match sexp {
                            Expression::Atom(_) => false,
                            Expression::List(_) => true,
                        })
                        .reduce(|acc, i| acc || i)
                        .unwrap_or(false);
                    match has_depth && v.len() > 1 {
                        true => {
                            format!(
                                "({}{}{}{}{})",
                                v.first().unwrap().to_pretty(width),
                                line_break,
                                indent,
                                v[1..]
                                    .iter()
                                    .map(|sexp| sexp
                                        .to_pretty(width)
                                        .split("\n")
                                        .map(|s| s.to_string())
                                        .reduce(|l, r| format!(
                                            "{}{}{}{}",
                                            l, line_break, indent, r
                                        ))
                                        .unwrap_or("".to_string()))
                                    .reduce(|l, r| format!("{}{}{}{}", l, line_break, indent, r))
                                    .unwrap_or("".to_string()),
                                final_break,
                            )
                        }
                        false => {
                            format!(
                                "({})",
                                v.iter()
                                    .map(|sexp| sexp.to_pretty(width))
                                    .reduce(|l, r| format!("{} {}", l, r))
                                    .unwrap_or("".to_string())
                            )
                        }
                    }
                }
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom() {
        assert_eq!(Expression::new("foo").to_string(), "foo");
    }

    #[test]
    fn test_list() {
        assert_eq!(
            Expression::new(vec![Expression::new("foo"), Expression::new("bar"),]).to_string(),
            "(foo bar)"
        );
    }

    #[test]
    fn test_nested_list() {
        assert_eq!(
            Expression::new(vec![
                Expression::new("foo"),
                Expression::new(vec![Expression::new("bar"), Expression::new("baz"),]),
                Expression::new(vec![Expression::new("bar"), Expression::new("baz"),]),
            ])
            .to_string(),
            "(foo (bar baz) (bar baz))"
        );
    }

    #[test]
    fn test_very_nested_list() {
        assert_eq!(
            Expression::new(vec![
                Expression::new("foo"),
                Expression::new(vec![
                    Expression::new("bar"),
                    Expression::new(vec![
                        Expression::new("baz"),
                        Expression::new(vec![
                            Expression::new("foo"),
                            Expression::new(vec![Expression::new("bar"), Expression::new("baz"),]),
                            Expression::new(vec![Expression::new("bar"), Expression::new("baz"),]),
                        ])
                    ]),
                ]),
            ])
            .to_string(),
            "(foo (bar (baz (foo (bar baz) (bar baz)))))"
        );
    }

    #[test]
    fn test_atom_to_pretty() {
        assert_eq!(Expression::new("foo").to_pretty(4), "foo");
    }

    #[test]
    fn test_list_to_pretty() {
        assert_eq!(
            Expression::new(vec![Expression::new("foo"), Expression::new("bar"),]).to_pretty(4),
            "(foo bar)"
        );
    }

    #[test]
    fn test_nested_list_to_pretty() {
        assert_eq!(
            Expression::new(vec![
                Expression::new("foo"),
                Expression::new(vec![Expression::new("bar"), Expression::new("baz"),]),
                Expression::new(vec![Expression::new("bar"), Expression::new("baz"),]),
            ])
            .to_pretty(4),
            "(foo
    (bar baz)
    (bar baz)
)"
        );
    }

    #[test]
    fn test_very_nested_list_to_pretty() {
        assert_eq!(
            Expression::new(vec![
                Expression::new("foo"),
                Expression::new(vec![
                    Expression::new("bar"),
                    Expression::new(vec![
                        Expression::new("baz"),
                        Expression::new(vec![
                            Expression::new("foo"),
                            Expression::new(vec![Expression::new("bar"), Expression::new("baz"),]),
                            Expression::new(vec![Expression::new("bar"), Expression::new("baz"),]),
                        ])
                    ]),
                ]),
            ])
            .to_pretty(4),
            "(foo
    (bar
        (baz
            (foo
                (bar baz)
                (bar baz)
            )
        )
    )
)"
        );
    }

    #[test]
    fn test_empty_module_to_bin() {
        assert_eq!(
            Expression::new(vec![Expression::new("module"),])
                .to_bin()
                .unwrap(),
            vec!(0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00) // \0asm magic and version 1
        )
    }
}
