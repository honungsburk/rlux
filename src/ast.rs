#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    Grouping(Box<Expression>),
    True,
    False,
    Nil,
    Unary(UnOp, Box<Expression>),
    Binary(Box<Expression>, BinOp, Box<Expression>),
}

impl StructuralPrinter for Expression {
    fn print_structural(&self) -> String {
        match self {
            Expression::Number(n) => n.to_string(),
            Expression::String(s) => format!("\"{}\"", s),
            Expression::Nil => "nil".to_string(),
            Expression::True => "true".to_string(),
            Expression::False => "false".to_string(),
            Expression::Grouping(expr) => format!("({})", expr.print_structural()),
            Expression::Unary(op, expr) => {
                format!("({}{})", op.print_structural(), expr.print_structural())
            }
            Expression::Binary(left, op, right) => format!(
                "({} {} {})",
                left.print_structural(),
                op.print_structural(),
                right.print_structural()
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Not,
    Minus,
}

impl UnOp {
    pub fn print(&self) -> String {
        let s = match self {
            UnOp::Not => "!",
            UnOp::Minus => "-",
        };
        return s.to_string();
    }
}

impl StructuralPrinter for UnOp {
    fn print_structural(&self) -> String {
        self.print()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Equals,
    NotEquals,
    Less,
    LessOrEquals,
    Greater,
    GreaterOrEquals,
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl BinOp {
    fn print(&self) -> String {
        let s = match self {
            BinOp::Equals => "==",
            BinOp::NotEquals => "!=",
            BinOp::Less => "<",
            BinOp::LessOrEquals => "<=",
            BinOp::Greater => ">",
            BinOp::GreaterOrEquals => ">=",
            BinOp::Plus => "+",
            BinOp::Minus => "-",
            BinOp::Multiply => "*",
            BinOp::Divide => "/",
        };
        s.to_string()
    }
}

impl StructuralPrinter for BinOp {
    fn print_structural(&self) -> String {
        self.print()
    }
}

pub trait StructuralPrinter {
    fn print_structural(&self) -> String;
}
