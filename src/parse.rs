use sexp::Sexp;
use std::fmt;
use thiserror::Error;

#[derive(Clone, PartialEq)]
pub enum Exp {
    // Integer
    Int(i64),

    // Float
    Float(f64),

    // Symbolic identifier
    Id(String),

    // Addition; lhs and rhs must resolve to a Num
    Add {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    Sub {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    // Multiplication; lhs and rhs must resolve to a Num
    Mult {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    Div {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    // Lambda function
    Lambda {
        arg: String,
        body: Box<Exp>,
    },

    // Application of a function
    App {
        func: Box<Exp>,
        arg: Box<Exp>,
    },

    // Conditional; cond must resolve to a Bool; resolves to lhs when cond is true, otherwise rhs
    // lhs and rhs must resolve to the same type
    If {
        cond: Box<Exp>,
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    // Equality; lhs and rhs must resolve to numbers
    Eq {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    Gt {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    Ge {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    Lt {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    Le {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    // Sequence of expressions; resolves to the last expression
    Begin(Vec<Exp>),

    // Boolean
    Bool(bool),

    // Immutable ref; can only be made on boxes; see borrow checking rules for more
    Ref(Box<Exp>),

    // Mutable ref; can only be made on boxes; see borrow checking rules for more
    MutRef(Box<Exp>),

    // Boxed value; can be borrowed as a ref; represents a heap-allocated value
    Box(Box<Exp>),

    // Get the value stored in a box
    Unbox(Box<Exp>),

    // Get the value stored in a ref
    Deref(Box<Exp>),

    // Set the value stored in a mutable ref
    Set {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    // Print the value of a num or bool to stdout
    Display(Box<Exp>),

    // Print the result of Exp.pp to stdout
    Debug(Box<Exp>),
}

impl fmt::Debug for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Int(n) => write!(f, "Int({})", n),
            Exp::Float(n) => write!(f, "Float({})", n),
            Exp::Id(s) => write!(f, "Id({})", s),
            Exp::Add { lhs, rhs } => write!(f, "Add({:?}, {:?})", lhs, rhs),
            Exp::Sub { lhs, rhs } => write!(f, "Sub({:?}, {:?})", lhs, rhs),
            Exp::Mult { lhs, rhs } => write!(f, "Mult({:?}, {:?})", lhs, rhs),
            Exp::Div { lhs, rhs } => write!(f, "Div({:?}, {:?})", lhs, rhs),
            Exp::Lambda { arg: symbol, body } => write!(f, "Lambda({}, {:?})", symbol, body),
            Exp::App { func, arg } => write!(f, "App({:?}, {:?})", func, arg),
            Exp::If { cond, lhs, rhs } => write!(f, "If({:?}, {:?}, {:?})", cond, lhs, rhs),
            Exp::Eq { lhs, rhs } => write!(f, "Eq({:?}, {:?})", lhs, rhs),
            Exp::Gt { lhs, rhs } => write!(f, "Gt({:?}, {:?})", lhs, rhs),
            Exp::Ge { lhs, rhs } => write!(f, "Ge({:?}, {:?})", lhs, rhs),
            Exp::Lt { lhs, rhs } => write!(f, "Lt({:?}, {:?})", lhs, rhs),
            Exp::Le { lhs, rhs } => write!(f, "Le({:?}, {:?})", lhs, rhs),
            Exp::Begin(exprs) => {
                write!(f, "Begin(")?;
                let mut iter = exprs.iter();
                if let Some(first) = iter.next() {
                    write!(f, "{:?}", first)?;
                    for expr in iter {
                        write!(f, ", {:?}", expr)?;
                    }
                }
                write!(f, ")")
            }
            Exp::Bool(b) => write!(f, "Bool({})", b),
            Exp::Ref(r) => write!(f, "Ref({:?})", r),
            Exp::MutRef(r) => write!(f, "MutRef({:?})", r),
            Exp::Box(b) => write!(f, "Box({:?})", b),
            Exp::Unbox(u) => write!(f, "Unbox({:?})", u),
            Exp::Deref(d) => write!(f, "Deref({:?})", d),
            Exp::Set { lhs, rhs } => write!(f, "Set({:?}, {:?})", lhs, rhs),
            Exp::Display(d) => write!(f, "Display({:?})", d),
            Exp::Debug(d) => write!(f, "Debug({:?})", d),
        }
    }
}

impl TryFrom<&str> for Exp {
    type Error = ParseError;
    fn try_from(item: &str) -> Result<Self, Self::Error> {
        parse(sexp::parse(item)?)
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("This function is not yet implemented")]
    NotImplemented,
    #[error("Parsing error")]
    ParseError,
    #[error("Sexp syntax error")]
    SexpError(#[from] Box<sexp::Error>),
    #[error("Let assignment expressions must have the structure (<symbol> <body>)")]
    MalformedAssignment,
}

fn parse(s_exp: Sexp) -> Result<Exp, ParseError> {
    use sexp::Atom::{F, I, S};
    use sexp::Sexp::{Atom, List};
    use Exp::*;
    match s_exp {
        Atom(I(i)) => Ok(Int(i)),
        Atom(F(f)) => Ok(Float(f)),
        Atom(S(s)) if s == "true" => Ok(Bool(true)),
        Atom(S(s)) if s == "false" => Ok(Bool(false)),
        Atom(S(s)) => Ok(Id(s)),
        List(l) => parse_list(l),
    }
}

fn parse_list(list: Vec<Sexp>) -> Result<Exp, ParseError> {
    use sexp::Atom::S;
    use sexp::Sexp::Atom;
    use sexp::Sexp::List;
    use std::boxed::Box;
    use Exp::*;
    let first = list.first().ok_or(ParseError::NotImplemented)?;
    match (first, &list[1..]) {
        (Atom(S(func)), [lhs, rhs]) if func == "+" => Ok(Add {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [lhs, rhs]) if func == "-" => Ok(Sub {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [lhs, rhs]) if func == "*" => Ok(Mult {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [lhs, rhs]) if func == "/" => Ok(Div {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [lhs, rhs]) if func == "=" => Ok(Eq {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [lhs, rhs]) if func == "<" => Ok(Lt {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [lhs, rhs]) if func == ">" => Ok(Gt {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [lhs, rhs]) if func == "<=" => Ok(Le {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [lhs, rhs]) if func == ">=" => Ok(Ge {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [rest @ ..]) if func == "begin" => {
            let parsed_exprs: Result<Vec<Exp>, ParseError> =
                rest.iter().map(|expr| parse(expr.clone())).collect();
            Ok(Exp::Begin(parsed_exprs?))
        }
        (Atom(S(func)), [Atom(S(symbol)), body]) if func == "lambda" => Ok(Lambda {
            arg: symbol.to_string(),
            body: Box::new(parse(body.clone())?),
        }),
        (Atom(S(func)), [exp]) if func == "ref" => Ok(Ref(Box::new(parse(exp.clone())?))),
        (Atom(S(func)), [exp]) if func == "mut-ref" => Ok(MutRef(Box::new(parse(exp.clone())?))),
        (Atom(S(func)), [exp]) if func == "box" => Ok(Exp::Box(Box::new(parse(exp.clone())?))),
        (Atom(S(func)), [exp]) if func == "unbox" => Ok(Unbox(Box::new(parse(exp.clone())?))),
        (Atom(S(func)), [exp]) if func == "deref" => Ok(Deref(Box::new(parse(exp.clone())?))),
        (Atom(S(func)), [exp]) if func == "display" => Ok(Display(Box::new(parse(exp.clone())?))),
        (Atom(S(func)), [exp]) if func == "debug" => Ok(Debug(Box::new(parse(exp.clone())?))),
        (Atom(S(func)), [lhs, rhs]) if func == "set" => Ok(Set {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (func_exp, [arg]) if list.len() == 2 => Ok(App {
            // TODO: Check if len check is
            // necessary
            func: Box::new(parse(func_exp.clone())?),
            arg: Box::new(parse(arg.clone())?),
        }),
        (Atom(S(func)), [cond, lhs, rhs]) if func == "if" => Ok(If {
            cond: Box::new(parse(cond.clone())?),
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [List(l), body]) if func == "let" => match &l[..] {
            [Atom(S(arg)), val] => Ok(App {
                func: Box::new(Lambda {
                    arg: arg.to_string(),
                    body: Box::new(parse(body.clone())?),
                }),
                arg: Box::new(parse(val.clone())?),
            }),
            _ => Err(ParseError::MalformedAssignment),
        },
        _ => Err(ParseError::ParseError),
    }
}
