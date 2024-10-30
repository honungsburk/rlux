pub mod value;
pub mod run_time_error;
pub mod environment;
pub mod lib;

use std::collections::HashMap;
use std::mem;

pub use value::LuxValue;
pub use value::LuxCallable;
pub use run_time_error::RuntimeError;
pub use environment::Environment;

use crate::ast::*;
use crate::program::Program;

#[derive(Debug)]
pub struct Interpreter {
    globals: Environment,
    env: Environment,
    locals: HashMap<String, usize>,
}


impl Interpreter {

    pub fn new() -> Self {
        let mut globals = Environment::new();
        lib::load(&mut globals); 
        Self {
            env: globals.clone(),
            globals: globals,
            locals: HashMap::new()
        }
    }

    pub fn with_env(env: Environment) -> Self {
        Self {
            env: env.extend(),
            globals: env,
            locals: HashMap::new()
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<Option<LuxValue>, RuntimeError> {
        self.eval_stmts(&program.statements)
    }


    pub fn resolve_local(&mut self, id: &str, depth: usize) {
        self.locals.insert(id.to_string(), depth);
    }

    pub fn lookup_variable(&mut self, id: &str) -> Option<LuxValue> {
        if let Some(depth) = self.locals.get(id) {
            self.env.get_at(id,*depth)
        } else {
            self.globals.get(id)
        }
    }

    //
    // Statements
    //

    pub fn eval_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<Option<LuxValue>, RuntimeError>  {
        let mut last_val = None;
        for stmt in stmts {
            last_val = self.eval_stmt(stmt)?;
        }
        Ok(last_val)
    }

    pub fn eval_stmt_with(&mut self, stmt: &Stmt, new_env: Environment) -> Result<Option<LuxValue>, RuntimeError> {
        let old_env = mem::replace(&mut self.env, new_env);
        let result = self.eval_stmt(stmt);
        self.env = old_env;
        result
    }


    /// Run a statement and return the last value of the statement.
    /// 
    /// The return value is used by the repl to print the last value of the statement.
    fn eval_stmt(&mut self, stmt: &Stmt) -> Result<Option<LuxValue>, RuntimeError> {
        match stmt {
            Stmt::Return(expr) => {
                let value = self.eval_expr(expr)?;
                Err(RuntimeError::Return(value))
            }
            Stmt::Function(name, args, body) => {
                self.env.define(name.clone(), LuxValue::function(name.clone(), args.clone(), body.clone(), self.env.clone()));
                Ok(None)
            }
            Stmt::Expression(expr) => {self.eval_expr(expr).map(Some)},
            Stmt::Print(expr) => {
                let val = self.eval_expr(expr)?;
                println!("{}", val.to_string());
                Ok(None)
            }
            Stmt::Var(name, expr) => {
                let val = self.eval_expr(expr)?;
                self.env.define(name.clone(), val.clone());
                Ok(Some(val))
            }
            Stmt::Block(stmts) => {
                self.eval_block(stmts, self.env.extend())
            }
            Stmt::If(cond, then, else_) => {
                let cond_val = self.eval_expr(cond)?;
                if cond_val.is_truthy() {
                    self.eval_stmt(then)
                } else if let Some(else_) = else_ {
                    self.eval_stmt(else_)
                } else {
                    Ok(None)
                }
            }
            Stmt::While(cond, body) => {
                let mut last_val = None;
                while self.eval_expr(cond)?.is_truthy() {
                    last_val = self.eval_stmt(body)?;
                }
                Ok(last_val)
            }
        }
    }


    pub(crate) fn eval_block(&mut self, stmts: &Vec<Stmt>, new_env: Environment) -> Result<Option<LuxValue>, RuntimeError> {
        let old_env = mem::replace(&mut self.env, new_env);
        let result = self.eval_stmts(stmts);
        self.env = old_env;
        result
    }



    //
    // Expressions
    //

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<LuxValue, RuntimeError> {
        // TODO: Use a worklist algorithm to avoid stack overflow
        match expr {
            Expr::Call(callee, arguments) => {

                let callee = self.eval_expr(callee)?;
                let args = arguments
                    .iter()
                    .map(|expr| self.eval_expr(expr))
                    .collect::<Result<Vec<_>, _>>()?;

                let callable = match callee {
                    LuxValue::Callable(callable) => callable,
                    _ => {
                        return Err(RuntimeError::UnsupportedType(
                            format!(
                                "Type `{}` is not callable, can only call functions and classes",
                                callee.type_name()
                            )))
                    }
                };

                if callable.arity() != args.len() {
                    return Err(RuntimeError::UnsupportedType(format!(
                            "Expected {} arguments, but got {}",
                            callable.arity(),
                            args.len()
                    )));
                }
            
                callable.call(self, &args)
            }
            Expr::LogicalOr(left, right) => {
                let left_val = self.eval_expr(left)?;
                if left_val.is_truthy() {
                    Ok(left_val)
                } else {
                    self.eval_expr(right)
                }
            }
            Expr::LogicalAnd(left, right) => {
                let left_val = self.eval_expr(left)?;
                if !left_val.is_truthy() {
                    Ok(left_val)
                } else {
                    self.eval_expr(right)
                }
            }
            Expr::Assignment(name, expr) => {
                let val = self.eval_expr(expr)?;

                let success = if let Some(depth) = self.locals.get(name) {
                    self.env.assign_at(name.clone(), val.clone(), *depth)
                } else {
                    self.globals.assign(name.clone(), val.clone())
                };

                if success {
                    Ok(val)
                } else {
                    Err(RuntimeError::UndefinedVariable(name.clone()))
                }
            }
            Expr::Variable(name) => self.lookup_variable(name).ok_or(RuntimeError::UndefinedVariable(name.clone())),
            Expr::Number(n) => Ok(LuxValue::Number(*n)),
            Expr::String(s) => Ok(LuxValue::String(s.clone())),
            Expr::True => Ok(LuxValue::Boolean(true)),
            Expr::False => Ok(LuxValue::Boolean(false)),
            Expr::Nil => Ok(LuxValue::Nil),
            Expr::Unary(op, expr) => {
                let val = self.eval_expr(expr)?;
                match op {
                    UnaryOp::Negate => {
                        match val {
                            LuxValue::Number(n) => Ok(LuxValue::Number(-n)),
                            unexpected => Err(RuntimeError::UnsupportedType(format!(
                                "Bad type for unary `-` operator: `{}`",
                                unexpected.type_name()
                            )))
                        }
                    },
                    UnaryOp::Not => {
                        Ok(LuxValue::Boolean(!val.is_truthy()))
                    }
                }
            }
            Expr::Binary(left, op, right) => {
                
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;

                match op {
                    // Math
                    BinaryOp::Plus => match (left_val, right_val) {
                        (LuxValue::Number(left), LuxValue::Number(right)) => Ok(LuxValue::Number(left + right)),
                        (LuxValue::String(left), LuxValue::String(right)) => Ok(LuxValue::String(left + &right)),
                        (left, right) => Err(RuntimeError::UnsupportedType(
                            format!(
                                "Binary `+` operator can only operate over two numbers or two strings. \
                                Got types `{}` and `{}`",
                                left.type_name(),
                                right.type_name()
                        )
                        .into())),
                    },
                    BinaryOp::Minus => bin_number_operator!(left_val - right_val, op),
                    BinaryOp::Multiply => bin_number_operator!(left_val * right_val, op),
                    BinaryOp::Divide => {
                        if let LuxValue::Number(right_num) = right_val {
                            if right_num == 0.0 {
                                return Err(RuntimeError::DivideByZero("Cannot divide by zero".to_string()))
                            }
                        }
                        bin_number_operator!(left_val / right_val, op)
                    }

                    // Comparison
                    BinaryOp::Greater => bin_comparison_operator!(left_val > right_val, op),
                    BinaryOp::GreaterOrEquals => bin_comparison_operator!(left_val >= right_val, op),
                    BinaryOp::Less => bin_comparison_operator!(left_val < right_val, op),
                    BinaryOp::LessOrEquals => bin_comparison_operator!(left_val <= right_val, op),
                    BinaryOp::Equals => Ok(LuxValue::Boolean(left_val == right_val)),
                    BinaryOp::NotEquals => Ok(LuxValue::Boolean(left_val != right_val)),
                }
            }
            Expr::Grouping(expr) => self.eval_expr(expr),
        }
    }
    
}

macro_rules! bin_number_operator {
    ( $left:tt $op:tt $right:tt, $op_token:expr ) => {
        match ($left, $right) {
            (LuxValue::Number(left), LuxValue::Number(right)) => Ok(LuxValue::Number(left $op right)),
            (left, right) => Err(RuntimeError::UnsupportedType(format!(
                    "Binary `{}` operator can only operate over two numbers. \
                    Got types `{}` and `{}`",
                    stringify!($op),
                    left.type_name(),
                    right.type_name()
                ),
            )),
        }
    };
}
use bin_number_operator;

macro_rules! bin_comparison_operator {
    ( $left:tt $op:tt $right:tt, $op_token:expr ) => {
        match ($left, $right) {
            (LuxValue::Number(left), LuxValue::Number(right)) => Ok(LuxValue::Boolean(left $op right)),
            (LuxValue::String(left), LuxValue::String(right)) => Ok(LuxValue::Boolean(left $op right)),
            (left, right) => Err(RuntimeError::UnsupportedType(format!(
                    "Binary `{}` operator can only compare two numbers or two strings. \
                    Got types `{}` and `{}`",
                    stringify!($op),
                    left.type_name(),
                    right.type_name()
                )
            ).into()),
        }
    };
}
use bin_comparison_operator;