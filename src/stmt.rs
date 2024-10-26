use crate::expr::{Expr, StructuralPrinter};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Var(String, Expr),
    Block(Vec<Stmt>),
}

impl StructuralPrinter for Stmt {
    fn print_structural(&self) -> String {
        match self {
            Stmt::Expression(expr) => format!("Expr({})", expr.print_structural()),
            Stmt::Print(expr) => format!("PrintExpr({})", expr.print_structural()),
            Stmt::Var(name, expr) => format!("Var({}, {})", name, expr.print_structural()),
            Stmt::Block(stmts) => format!("Block({})", stmts.iter().map(|s| s.print_structural()).collect::<Vec<String>>().join(", ")),
            Stmt::If(cond, then, else_) => format!("If({}, {}, {})", cond.print_structural(), then.print_structural(), else_.as_ref().map(|e| e.print_structural()).unwrap_or("None".to_string())),
        }
    }
}
