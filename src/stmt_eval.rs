use crate::environment::Environment;
use crate::run_time_error::RunTimeError;
use crate::stmt::Stmt;
use crate::expr_eval::{run as eval_expr, Value};

/// Run a statement and return the last value of the statement.
/// The return value is used by the repl to print the last value of the statement.
pub fn run(stmt: &Stmt, env: &mut Environment) -> Result<Option<Value>, RunTimeError> {
    match stmt {
        Stmt::Expression(expr) => {eval_expr(expr, env).map(Some)},
        Stmt::Print(expr) => {
            let val = eval_expr(expr, env)?;
            println!("{}", val.to_string());
            Ok(None)
        }
        Stmt::Var(name, expr) => {
            let val = eval_expr(expr, env)?;
            env.define(name.clone(), val.clone());
            Ok(Some(val))
        }
        Stmt::Block(stmts) => {
            let mut env = env.extend();
            let mut last_val = None;
            for stmt in stmts {
                last_val = run(stmt, &mut env)?;
            }
            Ok(last_val)
        }
    }
}
