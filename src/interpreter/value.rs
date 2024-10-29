use core::fmt;
use std::{fmt::{Display, Debug}, rc::Rc};

use super::{Interpreter, RunTimeError};

pub trait LuxCallable: Display + Debug {
    fn call(self: Rc<Self>, interpreter: &mut Interpreter, args: &[LuxValue])
        -> Result<LuxValue, RunTimeError>;
    fn arity(&self) -> usize;
}


#[derive(Clone)]
pub enum LuxValue {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Callable(Rc<dyn LuxCallable>)
}

impl PartialEq for LuxValue {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl LuxValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            LuxValue::Nil => false,
            LuxValue::Boolean(b) => *b,
            _ => true,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            LuxValue::Nil => "nil".to_string(),
            LuxValue::Boolean(b) => b.to_string(),
            LuxValue::Number(n) => n.to_string(),
            LuxValue::String(s) => s.clone(),
            LuxValue::Callable(callable) => format!("{}", callable)
        }
    }

    pub fn type_name(&self) -> &'static str  {
        match self {
            LuxValue::Nil => "nil",
            LuxValue::Boolean(_) => "boolean",
            LuxValue::Number(_) => "number",
            LuxValue::String(_) => "string",
            LuxValue::Callable(_) => "callable"
        }
    }

    pub fn equals(&self, other: &LuxValue) -> bool {
        match (self, other) {
            (LuxValue::Nil, LuxValue::Nil) => true,
            (LuxValue::Boolean(l),LuxValue::Boolean(r) ) => l == r,
            (LuxValue::Number(l),LuxValue::Number(r) ) => l == r,
            (LuxValue::String(l),LuxValue::String(r) ) => l == r,
            (LuxValue::Callable(l),LuxValue::Callable(r)) => Rc::ptr_eq(l, r),
            _ => false,
        }
    }
}

impl Display for LuxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuxValue::Callable(fun) => Display::fmt(fun, f),
            LuxValue::Boolean(boolean) => Display::fmt(boolean, f),
            LuxValue::Number(number) => {
                if number.floor() == *number {
                    write!(f, "{:.0}", number)
                } else {
                    Display::fmt(number, f)
                }
            }
            LuxValue::String(string) => f.write_str(string),
            LuxValue::Nil => f.write_str("nil"),
        }
    }
}

impl Debug for LuxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuxValue::String(s) => write!(f, "\"{}\"", s),
            other => Display::fmt(other, f),
        }
    }
}
