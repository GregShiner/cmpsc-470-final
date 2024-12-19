use core::fmt;
use std::collections::HashMap;

use crate::parse::Exp;
use thiserror::Error;

type Location = usize;
type Env = HashMap<String, Value>;

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Closure { arg: String, body: Exp, env: Env },
    Box(Location),
    Ref(Box<Value>),
    MutRef(Box<Value>),
    Moved,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "Int({})", n),
            Value::Float(n) => write!(f, "Float({})", n),
            Value::Bool(b) => write!(f, "Bool({})", b),
            Value::Box(l) => write!(f, "Box({})", l),
            Value::Ref(v) => write!(f, "Ref({:?})", v),
            Value::MutRef(v) => write!(f, "MutRef({:?})", v),
            Value::Moved => write!(f, "Moved"),
            Value::Closure { arg, body, env } => {
                write!(
                    f,
                    "Closure(arg: {:?}, body: {:?}, env: {:?}",
                    arg, body, env
                )
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum InterpError {
    #[error("{0} is not yet implemented")]
    NotImplemented(String),
    #[error("Value is not a number")]
    NotANumber,
    #[error("Cannot perform operation on incompatible types")]
    IncompatibleTypes,
    #[error("Condition must be a boolean")]
    ConditionNotBoolean,
    #[error("If branches must have the same type")]
    BranchTypeMismatch,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("This type cannot be displayed. Please use 'debug' instead")]
    CantDisplay,
    #[error("Cannot find symbol '{0}'")]
    SymbolNotFound(String),
}

// Helper function for numeric operations that work on both ints and floats
fn apply_numeric_op<F, G>(
    lhs: Value,
    rhs: Value,
    int_op: F,
    float_op: G,
) -> Result<Value, InterpError>
where
    F: FnOnce(i64, i64) -> Value,
    G: FnOnce(f64, f64) -> Value,
{
    match (lhs, rhs) {
        (Value::Int(a), Value::Int(b)) => Ok(int_op(a, b)),
        (Value::Float(a), Value::Float(b)) => Ok(float_op(a, b)),
        (Value::Int(_), Value::Float(_)) | (Value::Float(_), Value::Int(_)) => {
            Err(InterpError::IncompatibleTypes)
        }
        _ => Err(InterpError::NotANumber),
    }
}

// Helper function for comparison operations
fn apply_comparison<F, G>(
    lhs: Value,
    rhs: Value,
    int_op: F,
    float_op: G,
) -> Result<Value, InterpError>
where
    F: FnOnce(i64, i64) -> bool,
    G: FnOnce(f64, f64) -> bool,
{
    apply_numeric_op(
        lhs,
        rhs,
        |a, b| Value::Bool(int_op(a, b)),
        |a, b| Value::Bool(float_op(a, b)),
    )
}

fn div(lhs: Value, rhs: Value) -> Result<Value, InterpError> {
    match (lhs, rhs) {
        (Value::Int(a), Value::Int(b)) => {
            if b == 0 {
                Err(InterpError::DivisionByZero)
            } else {
                Ok(Value::Int(a / b))
            }
        }
        (Value::Float(a), Value::Float(b)) => {
            if b == 0.0 {
                Err(InterpError::DivisionByZero)
            } else {
                Ok(Value::Float(a / b))
            }
        }
        (Value::Int(_), Value::Float(_)) | (Value::Float(_), Value::Int(_)) => {
            Err(InterpError::IncompatibleTypes)
        }
        _ => Err(InterpError::NotANumber),
    }
}

fn eq(lhs: Value, rhs: Value) -> Result<Value, InterpError> {
    match (lhs, rhs) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
        _ => Err(InterpError::NotANumber),
    }
}

fn check_same_type(v1: &Value, v2: &Value) -> bool {
    match (v1, v2) {
        (Value::Int(_), Value::Int(_))
        | (Value::Float(_), Value::Float(_))
        | (Value::Bool(_), Value::Bool(_))
        | (Value::Box(_), Value::Box(_))
        | (Value::Ref(_), Value::Ref(_))
        | (Value::MutRef(_), Value::MutRef(_)) => true,
        _ => false,
    }
}

#[allow(unused_variables)]
pub fn interp(exp: Exp, env: &mut Env) -> Result<Value, InterpError> {
    match exp {
        Exp::Int(i) => Ok(Value::Int(i)),
        Exp::Float(i) => Ok(Value::Float(i)),
        Exp::Bool(i) => Ok(Value::Bool(i)),
        Exp::Add { lhs, rhs } => apply_numeric_op(
            interp(*lhs, env)?,
            interp(*rhs, env)?,
            |a, b| Value::Int(a + b),
            |a, b| Value::Float(a + b),
        ),
        Exp::Sub { lhs, rhs } => apply_numeric_op(
            interp(*lhs, env)?,
            interp(*rhs, env)?,
            |a, b| Value::Int(a - b),
            |a, b| Value::Float(a - b),
        ),
        Exp::Mult { lhs, rhs } => apply_numeric_op(
            interp(*lhs, env)?,
            interp(*rhs, env)?,
            |a, b| Value::Int(a * b),
            |a, b| Value::Float(a * b),
        ),
        Exp::Div { lhs, rhs } => div(interp(*lhs, env)?, interp(*rhs, env)?),
        Exp::Eq { lhs, rhs } => eq(interp(*lhs, env)?, interp(*rhs, env)?),
        Exp::Gt { lhs, rhs } => apply_comparison(
            interp(*lhs, env)?,
            interp(*rhs, env)?,
            |a, b| a > b,
            |a, b| a > b,
        ),
        Exp::Ge { lhs, rhs } => apply_comparison(
            interp(*lhs, env)?,
            interp(*rhs, env)?,
            |a, b| a >= b,
            |a, b| a >= b,
        ),
        Exp::Lt { lhs, rhs } => apply_comparison(
            interp(*lhs, env)?,
            interp(*rhs, env)?,
            |a, b| a < b,
            |a, b| a < b,
        ),
        Exp::Le { lhs, rhs } => apply_comparison(
            interp(*lhs, env)?,
            interp(*rhs, env)?,
            |a, b| a <= b,
            |a, b| a <= b,
        ),
        Exp::If { cond, lhs, rhs } => {
            let cond_val = interp(*cond, env)?;
            match cond_val {
                Value::Bool(test) => {
                    let lhs_val = interp(*lhs, env)?;
                    let rhs_val = interp(*rhs, env)?;

                    if !check_same_type(&lhs_val, &rhs_val) {
                        return Err(InterpError::BranchTypeMismatch);
                    }

                    Ok(if test { lhs_val } else { rhs_val })
                }
                _ => Err(InterpError::ConditionNotBoolean),
            }
        }
        Exp::Debug(e) => {
            let v = interp(*e, env)?;
            print!("{:?}", v);
            Ok(v)
        }
        Exp::Display(e) => {
            let v = interp(*e, env)?;
            match v {
                Value::Int(i) => print!("{}", i),
                Value::Float(f) => print!("{}", f),
                Value::Bool(b) => print!("{}", b),
                _ => return Err(InterpError::CantDisplay),
            }
            Ok(v)
        }
        Exp::Id(s) => env
            .get(&s)
            .ok_or(InterpError::SymbolNotFound(s))
            .map(|v| v.clone()),
        Exp::Lambda { symbol, body } => Err(InterpError::NotImplemented("Lambda".to_string())),
        Exp::App { func, arg } => Err(InterpError::NotImplemented("App".to_string())),
        Exp::Begin(es) => Err(InterpError::NotImplemented("Begin".to_string())),
        Exp::Ref(b) => Err(InterpError::NotImplemented("Ref".to_string())),
        Exp::MutRef(b) => Err(InterpError::NotImplemented("MutRef".to_string())),
        Exp::Box(v) => Err(InterpError::NotImplemented("Box".to_string())),
        Exp::Unbox(b) => Err(InterpError::NotImplemented("Unbox".to_string())),
        Exp::Deref(r) => Err(InterpError::NotImplemented("Deref".to_string())),
        Exp::Set { lhs, rhs } => Err(InterpError::NotImplemented("Set".to_string())),
    }
}
