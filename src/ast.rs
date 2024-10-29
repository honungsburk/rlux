pub mod expr;
pub mod stmt;


pub use expr::{Expr, UnaryOp, BinaryOp};
pub use stmt::Stmt;


pub trait StructuralPrinter {
    fn print_structural(&self) -> String;
}
