use crate::expr::{Expr, StructuralPrinter};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Var(String, Expr),
    Block(Vec<Stmt>),
}

impl Stmt {
    pub fn expression(expr: Expr) -> Self {
        Stmt::Expression(expr)
    }
    pub fn print(expr: Expr) -> Self {
        Stmt::Print(expr)
    }
    pub fn var(name: String, expr: Expr) -> Self {
        Stmt::Var(name, expr)
    }
    pub fn block(stmts: Vec<Stmt>) -> Self {
        Stmt::Block(stmts)
    }
    pub fn if_(cond: Expr, then: Stmt, else_: Option<Stmt>) -> Self {
        Stmt::If(cond, Box::new(then), else_.map(Box::new))
    }
    pub fn while_(cond: Expr, body: Stmt) -> Self {
        Stmt::While(cond, Box::new(body))
    }
}

impl StructuralPrinter for Stmt {
    fn print_structural(&self) -> String {
        match self {
            Stmt::Expression(expr) => format!("Expr({})", expr.print_structural()),
            Stmt::Print(expr) => format!("PrintExpr({})", expr.print_structural()),
            Stmt::Var(name, expr) => format!("Var({}, {})", name, expr.print_structural()),
            Stmt::Block(stmts) => format!("Block({})", stmts.iter().map(|s| s.print_structural()).collect::<Vec<String>>().join(", ")),
            Stmt::If(cond, then, else_) => format!("If({}, {}, {})", cond.print_structural(), then.print_structural(), else_.as_ref().map(|e| e.print_structural()).unwrap_or("None".to_string())),
            Stmt::While(cond, body) => format!("While({}, {})", cond.print_structural(), body.print_structural()),
        }
    }
}
