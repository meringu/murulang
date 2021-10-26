pub const TYPE_I32: &'static str = "i32";

pub fn indent(wat: String, dent: usize) -> String {
    wat.split("\n").collect::<Vec<&str>>().join(&format!("\n{}", " ".repeat(dent)))
}

pub fn module(inner: Vec<String>) -> String {
    format!(
r#"(module
    {}
)"#, indent(inner.join("\n\n"), 4))
}

pub fn import(module: &str, function_name: &str, import_as: &str, params: Vec<&str>, result: Option<&str>) -> String {
    let res = match result {
        Some(s) => format!(" (result {})", s),
        None => "".to_string(),
    };
    format!(r#"(import "{}" "{}"
    (func ${} (param {}){})
)"#, module, function_name, import_as, params.join(" "), res)
}

pub fn export(name: &str, wat: String) -> String {
    format!(r#"(export "{}" {})"#, name, wat)
}

pub fn store(addr: String, val: String) -> String {
    format!("(i32.store {} {})", addr, val)
}

pub fn get_local(index: i32) -> String {
    format!("(get_local {})", index)
}

pub fn memory(i: i32) -> String {
    format!("(memory {})", i)
}

pub fn function(name: &str, export: Option<&str>, params: Option<Vec<String>>, result: Option<String>, inner: Vec<String>) -> String {
    let exp = match export {
        Some(s) => format!(r#" (export "{}")"#, s),
        None => "".to_string(),
    };

    let p = match params {
        Some(p) => {
            if p.len() == 0 {
                "".to_string()
            } else {
                format!("(param {})", p.join(" "))
            }
        },
        None => "".to_string(),
    };

    let res = match result {
        Some(s) => format!(" (result {})", s),
        None => "".to_string(),
    };

    format!(r#"(func ${}{}{}{}
    {}
)"#, name, exp, p, res, indent(inner.join("\n"), 4))
}

pub fn call(refr: &str, args: Vec<String>) -> String {
    match args.len() {
        0 => format!("(call ${})", refr),
        _ => format!("(call ${}
    {}
)", refr, indent(args.join("\n"), 4)),
    }
}

pub fn drop() -> String {
    String::from("drop")
}

pub fn const_i32(val: i32) -> String {
    format!("(i32.const {})", val)
}

pub fn i32_add(left: String, right: String) -> String {
    format!("(i32.add
    {}
    {}
)", indent(left, 4), indent(right, 4))
}

pub fn i32_ne(left: String, right: String) -> String {
    format!("(i32.ne
    {}
    {}
)", indent(left, 4), indent(right, 4))
}

pub fn i32_rem_u(left: String, right: String) -> String {
    format!("(i32.rem_u
    {}
    {}
)", indent(left, 4), indent(right, 4))
}

pub fn i32_div_u(left: String, right: String) -> String {
    format!("(i32.div_u
    {}
    {}
)", indent(left, 4), indent(right, 4))
}

pub fn control_if(result: Option<String>, condition: String, truthy: String, falsy: Option<String>) -> String {
    let res = match result {
        Some(s) => format!(" (result {})", s),
        None => "".to_string(),
    };

    let f = match falsy {
        Some(f) => format!("
(else
    {}
)", indent(f, 4)),
        None => "".to_string(),
    };

    format!("(if{}
    {}
    (then
        {}
    ){}
)", res, indent(condition, 4), indent(truthy, 8), indent(f, 4))
}