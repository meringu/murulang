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
                                "({}\n    {}\n)",
                                v.first().unwrap().to_string(),
                                v[1..]
                                    .iter()
                                    .map(|sexp| sexp
                                        .to_string()
                                        .split("\n")
                                        .map(|s| s.to_string())
                                        .reduce(|l, r| format!("{}\n    {}", l, r))
                                        .unwrap_or("".to_string()))
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
    pub fn constant(&self, val: &str) -> SExpresion {
        SExpresion::List(vec![
            SExpresion::Atom(format!("{}.const", self)),
            SExpresion::Atom(val.to_string()),
        ])
    }

    pub fn store(&self, align: u32, offset: u32) -> SExpresion {
        SExpresion::List(vec![
            SExpresion::Atom(format!("{}.store", self)),
            SExpresion::Atom(align.to_string()),
            SExpresion::Atom(offset.to_string()),
        ])
    }
}

pub fn call(func: &str, args: Vec<SExpresion>) -> SExpresion {
    let mut l = vec![SExpresion::Atom(format!("call ${}", func))];

    for arg in args.into_iter() {
        l.push(arg)
    }

    return SExpresion::List(l);
}

pub fn export(name: &str, inner: Option<SExpresion>) -> SExpresion {
    let mut l = vec![
        SExpresion::Atom("export".to_string()),
        SExpresion::Atom(format!("\"{}\"", name)),
    ];

    if let Some(i) = inner {
        l.push(i);
    }

    SExpresion::List(l)
}

pub fn func(
    name: &str,
    export: Option<SExpresion>,
    params: Option<SExpresion>,
    result: Option<SExpresion>,
    body: Vec<SExpresion>,
) -> SExpresion {
    let e = match export {
        Some(s) => format!(" {}", s),
        _ => "".to_string(),
    };

    let mut sig = format!("func ${}{}", name, e);
    if let Some(p) = params {
        sig = format!("{} {}", sig, p)
    }
    if let Some(r) = result {
        sig = format!("{} {}", sig, r)
    }
    let mut l = vec![SExpresion::Atom(sig)];

    for b in body {
        l.push(b);
    }

    SExpresion::List(l)
}

pub fn import(
    module: &str,
    function: &str,
    import_as: &str,
    params: Option<SExpresion>,
    result: Option<SExpresion>,
) -> SExpresion {
    SExpresion::List(vec![
        SExpresion::Atom(format!("import \"{}\" \"{}\"", module, function)),
        func(import_as, None, params, result, vec![]),
    ])
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

pub fn param(types: Vec<Types>) -> SExpresion {
    let mut l = vec![SExpresion::Atom("param".to_string())];

    for t in types.iter() {
        l.push(SExpresion::Atom(t.to_string()))
    }

    SExpresion::List(l)
}

pub fn result(t: Types) -> SExpresion {
    SExpresion::List(vec![
        SExpresion::Atom("result".to_string()),
        SExpresion::Atom(t.to_string()),
    ])
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
