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
    Plus {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    // Multiplication; lhs and rhs must resolve to a Num
    Mult {
        lhs: Box<Exp>,
        rhs: Box<Exp>,
    },

    // Lambda function
    Lambda {
        symbol: String,
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
            Exp::Plus { lhs, rhs } => write!(f, "Plus({:?}, {:?})", lhs, rhs),
            Exp::Mult { lhs, rhs } => write!(f, "Mult({:?}, {:?})", lhs, rhs),
            Exp::Lambda { symbol, body } => write!(f, "Lambda({}, {:?})", symbol, body),
            Exp::App { func, arg } => write!(f, "App({:?}, {:?})", func, arg),
            Exp::If { cond, lhs, rhs } => write!(f, "If({:?}, {:?}, {:?})", cond, lhs, rhs),
            Exp::Eq { lhs, rhs } => write!(f, "Eq({:?}, {:?})", lhs, rhs),
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
    use std::boxed::Box;
    use Exp::*;
    let first = list.first().ok_or(ParseError::NotImplemented)?;
    match (first, &list[1..]) {
        (Atom(S(func)), [lhs, rhs]) if func == "+" => Ok(Plus {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [lhs, rhs]) if func == "*" => Ok(Mult {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [lhs, rhs]) if func == "=" => Ok(Eq {
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [cond, lhs, rhs]) if func == "if" => Ok(If {
            cond: Box::new(parse(cond.clone())?),
            lhs: Box::new(parse(lhs.clone())?),
            rhs: Box::new(parse(rhs.clone())?),
        }),
        (Atom(S(func)), [rest @ ..]) if func == "begin" => {
            let parsed_exprs: Result<Vec<Exp>, ParseError> =
                rest.iter().map(|expr| parse(expr.clone())).collect();
            Ok(Exp::Begin(parsed_exprs?))
        }
        (Atom(S(func)), [Atom(S(symbol)), body]) if func == "lambda" => Ok(Lambda {
            symbol: symbol.to_string(),
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
        _ => Err(ParseError::ParseError),
    }
}

#[cfg(test)]
mod tests {
    // Seperate modules defined in a file, by default, will not have in scope values defined in the
    // file, but outside of the module. This is big fancy words for "Other things defined in this
    // file will not be availble in here because this is a seperate module (denoted by the mod
    // keyword)". This line just brings those things into scope.
    use super::*;

    #[test]
    fn debug_id_test() {
        let word = String::from("Word");
        let exp: Exp = Exp::Id(word);
        assert_eq!(format!("{:?}", exp), "Id(Word)");
    }

    /// Checks that a Num expression gets correctly formatted
    #[test]
    fn debug_num_test() {
        // Instantiates a number expression containing a 5
        let exp = Exp::Int(5);
        // format! in this case will convert an object into its debug representation as defined in
        // the fmt function.
        // The assert will panic if the Exp object does not format correctly
        assert_eq!(format!("{:?}", exp), "Int(5)");
    }
    #[test]
    fn debug_plus_test() {
        let lhs: Exp = Exp::Int(5);
        let rhs: Exp = Exp::Int(8);
        let plus_exp = Exp::Plus {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };

        assert_eq!(format!("{:?}", plus_exp), "Plus(Int(5), Int(8))");
    }

    #[test]
    fn debug_mult_test() {
        let lhs: Exp = Exp::Int(5);
        let rhs: Exp = Exp::Int(8);
        let mult_exp = Exp::Mult {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };

        assert_eq!(format!("{:?}", mult_exp), "Mult(Int(5), Int(8))");
    }
    #[test]
    fn debug_lambda_test() {
        let symbol = String::from("Word");
        let body: Exp = Exp::Int(8);
        let lambda_exp = Exp::Lambda {
            symbol,
            body: Box::new(body),
        };

        assert_eq!(format!("{:?}", lambda_exp), "Lambda(Word, Int(8))");
    }
    #[test]
    fn debug_if_test() {
        let cond: Exp = Exp::Bool(true);
        let lhs: Exp = Exp::Int(5);
        let rhs: Exp = Exp::Int(8);
        let if_exp = Exp::If {
            cond: Box::new(cond),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };

        assert_eq!(format!("{:?}", if_exp), "If(Bool(true), Int(5), Int(8))");
    }
    #[test]
    fn debug_eq_test() {
        let lhs: Exp = Exp::Int(5);
        let rhs: Exp = Exp::Int(8);
        let eq_exp = Exp::Eq {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };

        assert_eq!(format!("{:?}", eq_exp), "Eq(Int(5), Int(8))");
    }
}
