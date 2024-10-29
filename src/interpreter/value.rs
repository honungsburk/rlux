use core::fmt;
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use super::{Environment, Interpreter, RuntimeError, Stmt};

pub trait LuxCallable: Display + Debug {
    fn call(
        self: Rc<Self>,
        interpreter: &mut Interpreter,
        args: &[LuxValue],
    ) -> Result<LuxValue, RuntimeError>;
    fn arity(&self) -> usize;
}

#[derive(Clone)]
pub enum LuxValue {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Callable(Rc<dyn LuxCallable>),
}

impl PartialEq for LuxValue {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl LuxValue {
    pub fn nil() -> Self {
        LuxValue::Nil
    }

    pub fn t() -> Self {
        LuxValue::Boolean(true)
    }

    pub fn f() -> Self {
        LuxValue::Boolean(false)
    }

    pub fn number(n: f64) -> Self {
        LuxValue::Number(n)
    }

    pub fn string(s: String) -> Self {
        LuxValue::String(s)
    }

    pub fn callable<T>(callable: T) -> Self
    where
        T: LuxCallable + 'static,
    {
        LuxValue::Callable(Rc::new(callable))
    }

    pub fn native_function(
        name: &'static str,
        arity: usize,
        fn_ptr: fn(args: &[LuxValue]) -> Result<LuxValue, RuntimeError>,
    ) -> Self {
        LuxValue::callable(NativeFunction {
            name: name,
            fn_ptr: fn_ptr,
            arity: arity,
        })
    }

    pub fn function(
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
        env: Environment
    ) -> Self {
        Self::callable(LuxFunction {
            decl: Rc::new(FunDecl {
                name, params, body
            }),
            env
        })

    }

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
            LuxValue::Callable(callable) => format!("{}", callable),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            LuxValue::Nil => "nil",
            LuxValue::Boolean(_) => "boolean",
            LuxValue::Number(_) => "number",
            LuxValue::String(_) => "string",
            LuxValue::Callable(_) => "callable",
        }
    }

    pub fn equals(&self, other: &LuxValue) -> bool {
        match (self, other) {
            (LuxValue::Nil, LuxValue::Nil) => true,
            (LuxValue::Boolean(l), LuxValue::Boolean(r)) => l == r,
            (LuxValue::Number(l), LuxValue::Number(r)) => l == r,
            (LuxValue::String(l), LuxValue::String(r)) => l == r,
            (LuxValue::Callable(l), LuxValue::Callable(r)) => Rc::ptr_eq(l, r),
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

// Native Function 

/// Function provided by the interpreter. Used by the standard library.
pub struct NativeFunction {
    pub name: &'static str,
    pub fn_ptr: fn(args: &[LuxValue]) -> Result<LuxValue, RuntimeError>,
    pub arity: usize,
}

impl LuxCallable for NativeFunction {
    fn call(
        self: Rc<Self>,
        _: &mut Interpreter,
        args: &[LuxValue],
    ) -> Result<LuxValue, RuntimeError> {
        (self.fn_ptr)(args)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fun (native) {}>", self.name)
    }
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeFunction")
            .field("name", &self.name)
            .field("fn_ptr", &"fn_ptr")
            .field("arity", &self.arity)
            .finish()
    }
}

// Function 


#[derive(Debug, Clone)]
pub struct FunDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub struct LuxFunction {
    pub decl: Rc<FunDecl>,
    pub env: Environment,
}

impl LuxCallable for LuxFunction {
    fn call(
        self: Rc<Self>,
        interpreter: &mut Interpreter,
        args: &[LuxValue],
    ) -> Result<LuxValue, RuntimeError> {
        let mut env = Environment::extend(&self.env);
        for (param, value) in self.decl.params.iter().zip(args) {
            env.define(param.clone(), value.clone());
        }
        let real_returned_value = match interpreter.eval_stmt_with(&self.decl.body, env) {
            Ok(_) => LuxValue::Nil,
            Err(RuntimeError::Return(value)) => value,
            Err(other) => return Err(other),
        };
        Ok(real_returned_value)
    }

    fn arity(&self) -> usize {
        self.decl.params.len()
    }
}

impl Display for LuxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fun {}>", self.decl.name)
    }
}