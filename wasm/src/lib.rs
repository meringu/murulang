pub enum SExpresion {
    Atom(String),
    List(Vec<SExpresion>),
}

impl ToString for SExpresion {
    fn to_string(&self) -> String {
        match self {
            SExpresion::Atom(s) => s.to_string(),
            SExpresion::List(v) => {
                let has_depth = v.iter().map(|sexp| match sexp {
                    SExpresion::Atom(_) => false,
                    SExpresion::List(_) => true,
                }).reduce(|acc, i| acc || i).unwrap_or(false);

                match has_depth && v.len() > 1 {
                    true => {
                        format!("({}\n    {}\n)",
                            v.first().unwrap().to_string(),
                            v[1..].iter().map(|sexp|
                                sexp.to_string()
                            ).reduce(|l, r|
                                format!("{}\n    {}", l, r)
                            ).unwrap_or("".to_string())
                        )
                    },
                    false => {
                        format!("({})", v.iter().map(|sexp|
                            sexp.to_string()
                        ).reduce(|l, r|
                            format!("{} {}", l, r)
                        ).unwrap_or("".to_string()))
                    }
                }
            },
        }
    }
}

pub enum Types {
    I32,
    I64,
    F32,
    F64,
}

impl Types {
    pub fn to_string(&self) -> String {
        match self {
            Types::I32 => "i32".to_string(),
            Types::I64 => "i64".to_string(),
            Types::F32 => "f32".to_string(),
            Types::F64 => "f64".to_string(),
        }
    }

    pub fn store(&self, align: u32, offset: u32) -> SExpresion {
        SExpresion::List(vec!(
            SExpresion::Atom(format!("{}.store", self.to_string())),
            SExpresion::Atom(align.to_string()),
            SExpresion::Atom(offset.to_string()),
        ))
    }
}

pub fn module(inner: Vec<SExpresion>) -> SExpresion {
    let mut inn = vec!(
        SExpresion::Atom("module".to_string())
    );

    for node in inner {
        inn.push(node)
    }

    SExpresion::List(inn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module() {
        assert_eq!(
            module(vec!(
                Types::I32.store(1, 2),
                Types::I32.store(3, 4)
            )).to_string(),
            r#"(module
    (i32.store 1 2)
    (i32.store 3 4)
)"#,
        );
    }

    #[test]
    fn test_store() {
        assert_eq!(
            Types::I32.store(1, 2).to_string(),
            "(i32.store 1 2)",
        );
    }
}