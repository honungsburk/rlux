#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Grouping(Box<Expr>),
    True,
    False,
    Nil,
    LogicalOr(Box<Expr>, Box<Expr>),
    LogicalAnd(Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Variable(String),
    Assignment(String, Box<Expr>),
}

impl Expr {
    pub fn number(n: f64) -> Expr {
        Expr::Number(n)
    }

    pub fn string(s: String) -> Expr {
        Expr::String(s)
    }

    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping(Box::new(expr))
    }

    pub fn true_expr() -> Expr {
        Expr::True
    }

    pub fn false_expr() -> Expr {
        Expr::False
    }

    pub fn nil() -> Expr {
        Expr::Nil
    }

    pub fn unary(op: UnaryOp, expr: Expr) -> Expr {
        Expr::Unary(op, Box::new(expr))
    }

    pub fn binary(left: Expr, op: BinaryOp, right: Expr) -> Expr {
        Expr::Binary(Box::new(left), op, Box::new(right))
    }

    pub fn logical_or(left: Expr, right: Expr) -> Expr {
        Expr::LogicalOr(Box::new(left), Box::new(right))
    }

    pub fn logical_and(left: Expr, right: Expr) -> Expr {
        Expr::LogicalAnd(Box::new(left), Box::new(right))
    }

    pub fn variable(name: String) -> Expr {
        Expr::Variable(name)
    }

    pub fn assignment(name: String, expr: Expr) -> Expr {
        Expr::Assignment(name, Box::new(expr))
    }
}

impl StructuralPrinter for Expr {
    fn print_structural(&self) -> String {
        match self {
            Expr::LogicalOr(left, right) => format!("({} or {})", left.print_structural(), right.print_structural()),
            Expr::LogicalAnd(left, right) => format!("({} and {})", left.print_structural(), right.print_structural()),
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Nil => "nil".to_string(),
            Expr::True => "true".to_string(),
            Expr::False => "false".to_string(),
            Expr::Grouping(expr) => format!("({})", expr.print_structural()),
            Expr::Unary(op, expr) => {
                format!("({}{})", op.print_structural(), expr.print_structural())
            }
            Expr::Binary(left, op, right) => format!(
                "({} {} {})",
                left.print_structural(),
                op.print_structural(),
                right.print_structural()
            ),
            Expr::Variable(name) => name.clone(),
            Expr::Assignment(name, expr) => format!("({} = {})", name, expr.print_structural()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Negate,
}

impl UnaryOp {
    pub fn print(&self) -> String {
        let s = match self {
            UnaryOp::Not => "!",
            UnaryOp::Negate => "-",
        };
        return s.to_string();
    }
}

impl StructuralPrinter for UnaryOp {
    fn print_structural(&self) -> String {
        self.print()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
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

impl BinaryOp {
    fn print(&self) -> String {
        let s = match self {
            BinaryOp::Equals => "==",
            BinaryOp::NotEquals => "!=",
            BinaryOp::Less => "<",
            BinaryOp::LessOrEquals => "<=",
            BinaryOp::Greater => ">",
            BinaryOp::GreaterOrEquals => ">=",
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
        };
        s.to_string()
    }
}

impl StructuralPrinter for BinaryOp {
    fn print_structural(&self) -> String {
        self.print()
    }
}

pub trait StructuralPrinter {
    fn print_structural(&self) -> String;
}
