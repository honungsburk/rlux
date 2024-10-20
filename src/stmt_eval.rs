use crate::environment::Environment;
use crate::run_time_error::RunTimeError;
use crate::stmt::Stmt;
use crate::expr_eval::run as eval_expr;


pub fn run(stmt: &Stmt, env: &mut Environment) -> Result<(), RunTimeError> {
    match stmt {
        Stmt::Expression(expr) => {eval_expr(expr, env)?; Ok(())},
        Stmt::Print(expr) => {
            let val = eval_expr(expr, env)?;
            println!("{}", val.to_string());
            Ok(())
        }
        Stmt::Var(name, expr) => {
            let val = eval_expr(expr, env)?;
            env.define(name.clone(), val);
            Ok(())
        }
    }
}
