use crate::{expr::{BinaryOp, Expr, UnaryOp}, run_time_error::RunTimeError};



#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
        }
    }
}

pub fn run(expr: &Expr) -> Result<Value, RunTimeError> {
    // TODO: Use a worklist algorithm to avoid stack overflow
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::True => Ok(Value::Boolean(true)),
        Expr::False => Ok(Value::Boolean(false)),
        Expr::Nil => Ok(Value::Nil),
        Expr::Unary(op, expr) => {
            let val = run(expr)?;
            match (op, val.clone()) {
                (UnaryOp::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
                (UnaryOp::Not, value) => Ok(Value::Boolean(!value.is_truthy())),
                _ => Err(RunTimeError::TypeError(format!(
                    "Invalid unary operation {:?} on value {:?}",
                    op, val
                ))),
            }
        }
        Expr::Binary(left, op, right) => {
            let left_val = run(left)?;
            let right_val = run(right)?;
            match (left_val.clone(), op, right_val.clone()) {
                (Value::Number(l), BinaryOp::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::Number(l), BinaryOp::Minus, Value::Number(r)) => Ok(Value::Number(l - r)),
                (Value::Number(l), BinaryOp::Multiply, Value::Number(r)) => {
                    Ok(Value::Number(l * r))
                }
                (Value::Number(l), BinaryOp::Divide, Value::Number(r)) => {
                    if r == 0.0 {
                        return Err(RunTimeError::DivideByZero(
                            "Cannot divide by zero".to_string(),
                        ));
                    }
                    Ok(Value::Number(l / r))
                }
                (Value::Number(l), BinaryOp::Greater, Value::Number(r)) => {
                    Ok(Value::Boolean(l > r))
                }
                (Value::Number(l), BinaryOp::GreaterOrEquals, Value::Number(r)) => {
                    Ok(Value::Boolean(l >= r))
                }
                (Value::Number(l), BinaryOp::Less, Value::Number(r)) => Ok(Value::Boolean(l < r)),
                (Value::Number(l), BinaryOp::LessOrEquals, Value::Number(r)) => {
                    Ok(Value::Boolean(l <= r))
                }
                (Value::Number(l), BinaryOp::Equals, Value::Number(r)) => {
                    Ok(Value::Boolean(l == r))
                }
                (Value::Number(l), BinaryOp::NotEquals, Value::Number(r)) => {
                    Ok(Value::Boolean(l != r))
                }
                (Value::Boolean(l), BinaryOp::Equals, Value::Boolean(r)) => {
                    Ok(Value::Boolean(l == r))
                }
                (Value::Boolean(l), BinaryOp::NotEquals, Value::Boolean(r)) => {
                    Ok(Value::Boolean(l != r))
                }
                (Value::String(l), BinaryOp::Plus, Value::String(r)) => Ok(Value::String(l + &r)),
                (Value::String(l), BinaryOp::Equals, Value::String(r)) => {
                    Ok(Value::Boolean(l == r))
                }
                (Value::String(l), BinaryOp::NotEquals, Value::String(r)) => {
                    Ok(Value::Boolean(l != r))
                }
                _ => Err(RunTimeError::TypeError(format!(
                    "Invalid binary operation {:?} on values {:?} and {:?}",
                    op, left_val, right_val
                ))),
            }
        }
        Expr::Grouping(expr) => run(expr),
    }
}
