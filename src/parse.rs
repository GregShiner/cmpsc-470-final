use sexp::Sexp;
use std::fmt;
use thiserror::Error;

#[derive(Clone)]
enum Exp {
    // Number
    Num(i32),

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
            Exp::Num(n) => write!(f, "Num({})", n),
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

#[derive(Error, Debug)]
enum ParseError {
    #[error("This method is not yet implemented")]
    NotImplemented,
}

fn parse(s_exp: Sexp) -> Result<Exp, ParseError> {
    use sexp::Atom::{F, I, S};
    use sexp::Sexp::{Atom, List};
    match s_exp {
        Atom(S("true")) => Ok(Exp::Bool(true)),
        _ => Err(ParseError::NotImplemented),
    }
}

#[cfg(test)]
mod tests {
    // Seperate modules defined in a file, by default, will not have in scope values defined in the
    // file, but outside of the module. This is big fancy words for "Other things defined in this
    // file will not be availble in here because this is a seperate module (denoted by the mod
    // keyword)". This line just brings those things into scope.
    use super::*;

    /// Checks that a Num expression gets correctly formatted
    #[test]
    fn debug_num_test() {
        // Instantiates a number expression containing a 5
        let exp = Exp::Num(5);
        // format! in this case will convert an object into its debug representation as defined in
        // the fmt function.
        // The assert will panic if the Exp object does not format correctly
        assert_eq!(format!("{:?}", exp), "Num(5)");
    }
}
