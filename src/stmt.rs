use crate::expr::{Expr, StructuralPrinter};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
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
        }
    }
}
