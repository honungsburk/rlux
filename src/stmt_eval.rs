use crate::run_time_error::RunTimeError;
use crate::stmt::Stmt;
use crate::expr_eval::{run as eval_expr, Value};


pub fn run(stmt: &Stmt) -> Result<Value, RunTimeError> {
    match stmt {
        Stmt::Expression(expr) => eval_expr(expr),
        Stmt::Print(expr) => {
            let val = eval_expr(expr)?;
            println!("{}", val.to_string());
            Ok(val)
        }
    }
}
