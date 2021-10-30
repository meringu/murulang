use std::error;
use std::fmt;
use wabt::wat2wasm;

pub enum SExpression {
    Atom(String),
    List(Vec<SExpression>),
}

impl fmt::Display for SExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SExpression::Atom(s) => s.to_string(),
                SExpression::List(v) => {
                    let has_depth = v
                        .iter()
                        .map(|sexp| match sexp {
                            SExpression::Atom(_) => false,
                            SExpression::List(_) => true,
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

impl SExpression {
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
    pub fn constant(&self, val: &str) -> SExpression {
        SExpression::List(vec![
            SExpression::Atom(format!("{}.const", self)),
            SExpression::Atom(val.to_string()),
        ])
    }

    pub fn store(&self, align: SExpression, offset: SExpression) -> SExpression {
        SExpression::List(vec![
            SExpression::Atom(format!("{}.store", self)),
            SExpression::Atom(align.to_string()),
            SExpression::Atom(offset.to_string()),
        ])
    }
}

pub fn call(func: &str, args: Vec<SExpression>) -> SExpression {
    let mut l = vec![SExpression::Atom(format!("call ${}", func))];

    for arg in args.into_iter() {
        l.push(arg)
    }

    return SExpression::List(l);
}

pub fn drop() -> SExpression {
    SExpression::Atom("drop".to_string())
}

pub fn export(name: &str, inner: Option<SExpression>) -> SExpression {
    let mut l = vec![
        SExpression::Atom("export".to_string()),
        SExpression::Atom(format!("\"{}\"", name)),
    ];

    if let Some(i) = inner {
        l.push(i);
    }

    SExpression::List(l)
}

pub fn func(
    name: &str,
    export: Option<SExpression>,
    params: Option<SExpression>,
    result: Option<SExpression>,
    body: Vec<SExpression>,
) -> SExpression {
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
    let mut l = vec![SExpression::Atom(sig)];

    for b in body {
        l.push(b);
    }

    SExpression::List(l)
}

pub fn get_local(local: &str) -> SExpression {
    SExpression::List(vec![
        SExpression::Atom("get_local".to_string()),
        SExpression::Atom(local.to_string()),
    ])
}

pub fn import(
    module: &str,
    function: &str,
    import_as: &str,
    params: Option<SExpression>,
    result: Option<SExpression>,
) -> SExpression {
    SExpression::List(vec![
        SExpression::Atom(format!("import \"{}\" \"{}\"", module, function)),
        func(import_as, None, params, result, vec![]),
    ])
}

pub fn memory(i: i32) -> SExpression {
    SExpression::List(vec![
        SExpression::Atom("memory".to_string()),
        SExpression::Atom(i.to_string()),
    ])
}

pub fn module(inner: Vec<SExpression>) -> SExpression {
    let mut inn = vec![SExpression::Atom("module".to_string())];

    for node in inner {
        inn.push(node)
    }

    SExpression::List(inn)
}

pub fn param(types: Vec<Types>) -> SExpression {
    let mut l = vec![SExpression::Atom("param".to_string())];

    for t in types.iter() {
        l.push(SExpression::Atom(t.to_string()))
    }

    SExpression::List(l)
}

pub fn result(t: Types) -> SExpression {
    SExpression::List(vec![
        SExpression::Atom("result".to_string()),
        SExpression::Atom(t.to_string()),
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
                SExpression::Atom("foo".to_string()),
                SExpression::List(vec!(
                    SExpression::Atom("bar".to_string()),
                    SExpression::Atom("baz".to_string()),
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
