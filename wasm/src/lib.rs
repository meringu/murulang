use std::error;
use std::fmt;
use wabt::wat2wasm;

pub enum SExpresion {
    Atom(String),
    List(Vec<SExpresion>),
}

impl fmt::Display for SExpresion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SExpresion::Atom(s) => s.to_string(),
                SExpresion::List(v) => {
                    let has_depth = v
                        .iter()
                        .map(|sexp| match sexp {
                            SExpresion::Atom(_) => false,
                            SExpresion::List(_) => true,
                        })
                        .reduce(|acc, i| acc || i)
                        .unwrap_or(false);

                    match has_depth && v.len() > 1 {
                        true => {
                            format!(
                                "({}\n)",
                                v.iter()
                                    .map(|sexp| sexp.to_string())
                                    .reduce(|l, r| format!("{}\n    {}", l, r))
                                    .unwrap_or("".to_string())
                            )
                        }
                        false => {
                            format!(
                                "({})",
                                v.iter()
                                    .map(|sexp| sexp.to_string())
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

impl SExpresion {
    pub fn bin(self) -> Result<Vec<u8>, Box<dyn error::Error>> {
        Ok(wat2wasm(self.to_string())?)
    }
}

pub enum Types {
    I32,
    I64,
    F32,
    F64,
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Types::I32 => "i32",
                Types::I64 => "i64",
                Types::F32 => "f32",
                Types::F64 => "f64",
            }
        )
    }
}

impl Types {
    pub fn store(&self, align: u32, offset: u32) -> SExpresion {
        SExpresion::List(vec![
            SExpresion::Atom(format!("{}.store", self)),
            SExpresion::Atom(align.to_string()),
            SExpresion::Atom(offset.to_string()),
        ])
    }
}

pub fn memory(i: i32) -> SExpresion {
    SExpresion::List(vec![
        SExpresion::Atom("memory".to_string()),
        SExpresion::Atom(i.to_string()),
    ])
}

pub fn module(inner: Vec<SExpresion>) -> SExpresion {
    let mut inn = vec![SExpresion::Atom("module".to_string())];

    for node in inner {
        inn.push(node)
    }

    SExpresion::List(inn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory() {
        assert_eq!(memory(1).to_string(), "(memory 1)",);
    }

    #[test]
    fn test_module() {
        assert_eq!(
            module(vec!(
                SExpresion::Atom("foo".to_string()),
                SExpresion::List(vec!(
                    SExpresion::Atom("bar".to_string()),
                    SExpresion::Atom("baz".to_string()),
                )),
            ))
            .to_string(),
            r#"(module
    foo
    (bar baz)
)"#,
        );
    }

    #[test]
    fn test_store() {
        assert_eq!(Types::I32.store(1, 2).to_string(), "(i32.store 1 2)",);
    }
}
