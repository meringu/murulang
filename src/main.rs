#[macro_use]
extern crate pest_derive;
extern crate from_pest;
#[macro_use]
extern crate pest_ast;
extern crate pest;

mod muru {
    #[derive(Parser)]
    #[grammar = "muru.pest"]
    pub struct Parser;
}

mod err {
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

    #[derive(Debug, Clone)]
    pub struct NotImplementedError<'a> {
        pub sub: &'a str,
    }

    impl<'a> std::fmt::Display for NotImplementedError<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "not implemented error: {}", self.sub)
        }
    }

    impl<'a> std::error::Error for NotImplementedError<'a> {}

    #[derive(Debug, Clone)]
    pub struct ArgumentError<'a> {
        pub function_name: &'a str,
        pub expected: usize,
        pub actual: usize,
    }

    impl<'a> std::fmt::Display for ArgumentError<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "argument error: {} expected: {}, got: {}", self.function_name, self.expected, self.actual)
        }
    }

    impl<'a> std::error::Error for ArgumentError<'a> {}
}

mod ast {
    use super::muru::Rule;
    use pest::Span;

    fn span_into_str(span: Span) -> &str {
        span.as_str()
    }

    #[derive(Debug, FromPest, Copy, Clone)]
    #[pest_ast(rule(Rule::int))]
    pub struct Int {
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        pub val: i64,
    }

    #[derive(Debug, FromPest, Copy, Clone)]
    #[pest_ast(rule(Rule::float))]
    pub struct Float {
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        pub val: f64,
    }

    #[derive(Debug, FromPest, Copy, Clone)]
    #[pest_ast(rule(Rule::bool))]
    pub struct Bool {
        #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
        pub val: bool,
    }

    #[derive(Debug, FromPest, Copy, Clone)]
    #[pest_ast(rule(Rule::literal))]
    pub enum Type {
        Int(Int),
        Float(Float),
        Bool(Bool),
    }

    impl std::fmt::Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Type::Int(v) => write!(f, "{}", v.val),
                Type::Float(v) => write!(f, "{}", v.val),
                Type::Bool(v) => write!(f, "{}", v.val),
            }
        }
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::variable))]
    pub struct Variable<'a> {
        #[pest_ast(outer(with(span_into_str)))]
        pub name: &'a str,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::add))]
    pub struct Add<'a> {
        #[pest_ast(outer(with(span_into_str)))]
        val: &'a str,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::subtract))]
    pub struct Subtract<'a> {
        #[pest_ast(outer(with(span_into_str)))]
        val: &'a str,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::multiply))]
    pub struct Multiply<'a> {
        #[pest_ast(outer(with(span_into_str)))]
        val: &'a str,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::divide))]
    pub struct Divide<'a> {
        #[pest_ast(outer(with(span_into_str)))]
        val: &'a str,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::eq))]
    pub struct Eq<'a> {
        #[pest_ast(outer(with(span_into_str)))]
        val: &'a str,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::neq))]
    pub struct Neq<'a> {
        #[pest_ast(outer(with(span_into_str)))]
        val: &'a str,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::operator))]
    pub enum Operator<'a> {
        Add(Add<'a>),
        Subtract(Subtract<'a>),
        Multiply(Multiply<'a>),
        Divide(Divide<'a>),
        Eq(Eq<'a>),
        Neq(Neq<'a>),
    }

    impl<'a> Operator<'a> {
        pub fn eval(
            &self,
            left: crate::ast::Type,
            right: crate::ast::Type,
        ) -> Result<crate::ast::Type, Box<dyn std::error::Error>> {
            match self {
                Operator::Add(_) => match left {
                    crate::ast::Type::Int(l) => match right {
                        crate::ast::Type::Int(r) => Ok(crate::ast::Type::Int(crate::ast::Int{val: l.val + r.val})),
                        crate::ast::Type::Float(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val as f64 + r.val})),
                        _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                    },
                    crate::ast::Type::Float(l) => match right {
                        crate::ast::Type::Int(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val + r.val as f64})),
                        crate::ast::Type::Float(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val + r.val})),
                        _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                    },
                    _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                },
                Operator::Subtract(_) => match left {
                    crate::ast::Type::Int(l) => match right {
                        crate::ast::Type::Int(r) => Ok(crate::ast::Type::Int(crate::ast::Int{val: l.val - r.val})),
                        crate::ast::Type::Float(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val as f64 - r.val})),
                        _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                    },
                    crate::ast::Type::Float(l) => match right {
                        crate::ast::Type::Int(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val - r.val as f64})),
                        crate::ast::Type::Float(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val - r.val})),
                        _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                    },
                    _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                },
                Operator::Multiply(_) => match left {
                    crate::ast::Type::Int(l) => match right {
                        crate::ast::Type::Int(r) => Ok(crate::ast::Type::Int(crate::ast::Int{val: l.val * r.val})),
                        crate::ast::Type::Float(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val as f64 * r.val})),
                        _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                    },
                    crate::ast::Type::Float(l) => match right {
                        crate::ast::Type::Int(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val * r.val as f64})),
                        crate::ast::Type::Float(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val * r.val})),
                        _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                    },
                    _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                },
                Operator::Divide(_) => match left {
                    crate::ast::Type::Int(l) => match right {
                        crate::ast::Type::Int(r) => Ok(crate::ast::Type::Int(crate::ast::Int{val: l.val / r.val})),
                        crate::ast::Type::Float(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val as f64 / r.val})),
                        _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                    },
                    crate::ast::Type::Float(l) => match right {
                        crate::ast::Type::Int(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val / r.val as f64})),
                        crate::ast::Type::Float(r) => Ok(crate::ast::Type::Float(crate::ast::Float{val: l.val / r.val})),
                        _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                    },
                    _ => Err(Box::new(crate::err::NotImplementedError{sub: "TODO"})),
                },
                Operator::Eq(_) => match left {
                    crate::ast::Type::Int(l) => match right {
                        crate::ast::Type::Int(r) => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: l.val == r.val})),
                        _ => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: false})),
                    },
                    crate::ast::Type::Float(l) => match right {
                        crate::ast::Type::Float(r) => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: l.val == r.val})),
                        _ => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: false})),
                    },
                    crate::ast::Type::Bool(l) => match right {
                        crate::ast::Type::Bool(r) => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: l.val == r.val})),
                        _ => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: false})),
                    },
                },
                Operator::Neq(_) => match left {
                    crate::ast::Type::Int(l) => match right {
                        crate::ast::Type::Int(r) => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: l.val != r.val})),
                        _ => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: true})),
                    },
                    crate::ast::Type::Float(l) => match right {
                        crate::ast::Type::Float(r) => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: l.val != r.val})),
                        _ => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: true})),
                    },
                    crate::ast::Type::Bool(l) => match right {
                        crate::ast::Type::Bool(r) => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: l.val != r.val})),
                        _ => Ok(crate::ast::Type::Bool(crate::ast::Bool{val: true})),
                    },
                },
            }
        }
    }
    
    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::argument))]
    pub enum Argument<'a> {
        Expression(Expression<'a>),
        Literal(Type),
        Variable(Variable<'a>),
    }

    impl<'a> Argument<'a> {
        pub fn eval(
            &self,
            globals: &std::collections::HashMap::<&str, crate::ast::Function>,
            locals: &std::collections::HashMap::<&str, crate::ast::Function>,
        ) -> Result<crate::ast::Type, Box<dyn std::error::Error>> {
            match self {
                Argument::Expression(e) => e.eval(globals, locals),
                Argument::Literal(t) => Ok(t.clone()),
                Argument::Variable(c) => crate::ast::Call{
                    variable: crate::ast::Variable{name: c.name},
                    args: vec!(),
                }.eval(globals, locals),
            }
        }
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::call))]
    pub struct Call<'a> {
        pub variable: Variable<'a>,
        pub args: Vec<Argument<'a>>,
    }

    impl<'a> Call<'a> {
        pub fn eval(
            &self,
            globals: &std::collections::HashMap::<&str, crate::ast::Function>,
            locals: &std::collections::HashMap::<&str, crate::ast::Function>,
        ) -> Result<crate::ast::Type, Box<dyn std::error::Error>> {
            let f = match locals.get(self.variable.name) {
                Some(f) => f,
                None => match globals.get(self.variable.name) {
                    Some(f) => f,
                    None => return Err(Box::new(crate::err::FunctionNotFoundError{name: "TODO"})),
                }
            };

            let mut args = vec!();
            for arg in &self.args {
                args.push(arg.eval(globals, locals)?);
            }

            f.call(globals, args)
        }
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::unary))]
    pub enum Unary<'a> {
        Expression(Expression<'a>),
        Literal(Type),
        Call(Call<'a>)
    }

    impl<'a> Unary<'a> {
        pub fn eval(
            &self,
            globals: &std::collections::HashMap::<&str, crate::ast::Function>,
            locals: &std::collections::HashMap::<&str, crate::ast::Function>,
        ) -> Result<crate::ast::Type, Box<dyn std::error::Error>> {
            match self {
                Unary::Expression(e) => e.eval(globals, locals),
                Unary::Literal(t) => Ok(t.clone()),
                Unary::Call(c) => c.eval(globals, locals),
            }
        }
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::binary))]
    pub struct Binary<'a> {
        pub left: Unary<'a>,
        pub operator: Operator<'a>,
        pub right: Unary<'a>,
    }

    impl<'a> Binary<'a> {
        pub fn eval(
            &self,
            globals: &std::collections::HashMap::<&str, crate::ast::Function>,
            locals: &std::collections::HashMap::<&str, crate::ast::Function>,
        ) -> Result<crate::ast::Type, Box<dyn std::error::Error>> {
            let left = self.left.eval(globals, locals)?;
            let right = self.right.eval(globals, locals)?;
            self.operator.eval(left, right)
        }
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::expression))]
    pub enum Expression<'a> {
        Unary(Box<Unary<'a>>),
        Binary(Box<Binary<'a>>),
    }

    impl<'a> Expression<'a> {
        pub fn eval(
            &self,
            globals: &std::collections::HashMap::<&str, crate::ast::Function>,
            locals: &std::collections::HashMap::<&str, crate::ast::Function>,
        ) -> Result<crate::ast::Type, Box<dyn std::error::Error>> {
            match self {
                Expression::Unary(u) => u.eval(globals, locals),
                Expression::Binary(b) => b.eval(globals, locals),
            }
        }
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::function))]
    pub struct Function<'a> {
        pub name: Variable<'a>,
        pub variables: Vec<Variable<'a>>,
        pub expr: Expression<'a>,
    }

    impl<'a> Function<'a> {
        pub fn call(
            &self,
            globals: &std::collections::HashMap::<&str, crate::ast::Function>,
            args: Vec<crate::ast::Type>
        ) -> Result<crate::ast::Type, Box<dyn std::error::Error>> {
            if args.len() != self.variables.len() {
                return Err(Box::new(crate::err::ArgumentError{
                    // function_name: self.name.name, // teach me how to lifetime
                    function_name: "TODO",
                    expected: self.variables.len(),
                    actual: args.len(),
                }));
            }

            let mut locals = std::collections::HashMap::<&str, crate::ast::Function>::new();
            for i in 0..self.variables.len() {
                locals.insert(
                    self.variables[i].name,
                    crate::ast::Function {
                        name: crate::ast::Variable{name: self.variables[i].name},
                        variables: vec!(),
                        expr: crate::ast::Expression::Unary(
                            Box::new(crate::ast::Unary::Literal(args[i].clone()))
                        )
                    }
                );
            }

            self.expr.eval(globals, &locals)
        }
    }
    

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::program))]
    pub struct Program<'a> {
        pub functions: Vec<Function<'a>>,
        eoi: EOI,
    }

    #[derive(Debug, FromPest)]
    #[pest_ast(rule(Rule::EOI))]
    struct EOI;
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    use ast::Program;
    use from_pest::FromPest;
    use pest::Parser;

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} progam.muru", args[0]);
        std::process::exit(1);
    }

    let source = String::from_utf8(std::fs::read(&args[1])?)?;
    let mut parse_tree = muru::Parser::parse(muru::Rule::program, &source)?;
    // println!("parse tree = {:#?}", parse_tree);
    let program: Program = Program::from_pest(&mut parse_tree).expect("infallible");
    // println!("syntax tree = {:#?}", program);

    let mut functions = std::collections::HashMap::<&str, ast::Function>::new();
    for f in program.functions.into_iter() {
        functions.insert(
            f.name.name,
            f
        );
    }

    let main = match functions.get("main") {
        Some(f) => f,
        None => {
            return Err(Box::new(err::FunctionNotFoundError{name: "main"}));
        },
    };

    let args = vec!();
    let res = main.call(&functions, args)?;

    println!("{}", res);

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
