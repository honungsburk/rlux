#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Grouping(Box<Expr>),
    True,
    False,
    Nil,
    Unary(UnOp, Box<Expr>),
    Binary(Box<Expr>, BinOp, Box<Expr>),
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

    pub fn unary(op: UnOp, expr: Expr) -> Expr {
        Expr::Unary(op, Box::new(expr))
    }

    pub fn binary(left: Expr, op: BinOp, right: Expr) -> Expr {
        Expr::Binary(Box::new(left), op, Box::new(right))
    }
}

impl StructuralPrinter for Expr {
    fn print_structural(&self) -> String {
        match self {
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
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Not,
    Negate,
}

impl UnOp {
    pub fn print(&self) -> String {
        let s = match self {
            UnOp::Not => "!",
            UnOp::Negate => "-",
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
