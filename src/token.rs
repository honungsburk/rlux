use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier(String),
    String(String),
    UnterminatedString,
    Number(f64),
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // End of file
    Eof,
    UnknownChar(char),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tt: TokenType = (*self).into();
        tt.fmt(f)
    }
}

impl Into<TokenType> for Token {
    fn into(self) -> TokenType {
        match self {
            Token::LeftParen => TokenType::LeftParen,
            Token::RightParen => TokenType::RightParen,
            Token::LeftBrace => TokenType::LeftBrace,
            Token::RightBrace => TokenType::RightBrace,
            Token::Comma => TokenType::Comma,
            Token::Dot => TokenType::Dot,
            Token::Minus => TokenType::Minus,
            Token::Plus => TokenType::Plus,
            Token::Semicolon => TokenType::Semicolon,
            Token::Slash => TokenType::Slash,
            Token::Star => TokenType::Star,
            Token::Bang => TokenType::Bang,
            Token::BangEqual => TokenType::BangEqual,
            Token::Equal => TokenType::Equal,
            Token::EqualEqual => TokenType::EqualEqual,
            Token::Greater => TokenType::Greater,
            Token::GreaterEqual => TokenType::GreaterEqual,
            Token::Less => TokenType::Less,
            Token::LessEqual => TokenType::LessEqual,
            Token::Identifier(_) => TokenType::Identifier,
            Token::String(_) => TokenType::String,
            Token::UnterminatedString => TokenType::UnterminatedString,
            Token::Number(_) => TokenType::Number,
            Token::And => TokenType::And,
            Token::Class => TokenType::Class,
            Token::Else => TokenType::Else,
            Token::False => TokenType::False,
            Token::Fun => TokenType::Fun,
            Token::For => TokenType::For,
            Token::If => TokenType::If,
            Token::Nil => TokenType::Nil,
            Token::Or => TokenType::Or,
            Token::Print => TokenType::Print,
            Token::Return => TokenType::Return,
            Token::Super => TokenType::Super,
            Token::This => TokenType::This,
            Token::True => TokenType::True,
            Token::Var => TokenType::Var,
            Token::While => TokenType::While,
            Token::Eof => TokenType::Eof,
            Token::UnknownChar(c) => TokenType::UnknownChar,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    UnterminatedString,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // End of file
    Eof,
    UnknownChar,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{}", "{"),
            TokenType::RightBrace => write!(f, "{}", "}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Identifier => write!(f, "identifier"),
            TokenType::String => write!(f, "string"),
            TokenType::Number => write!(f, "number"),
            TokenType::And => write!(f, "and"),
            TokenType::Class => write!(f, "class"),
            TokenType::Else => write!(f, "else"),
            TokenType::False => write!(f, "false"),
            TokenType::Fun => write!(f, "fun"),
            TokenType::For => write!(f, "for"),
            TokenType::If => write!(f, "if"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::Or => write!(f, "or"),
            TokenType::Print => write!(f, "print"),
            TokenType::Return => write!(f, "return"),
            TokenType::Super => write!(f, "super"),
            TokenType::This => write!(f, "this"),
            TokenType::True => write!(f, "true"),
            TokenType::Var => write!(f, "var"),
            TokenType::While => write!(f, "while"),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::UnterminatedString => write!(f, "unterminated-string"),
            TokenType::UnknownChar => write!(f, "unknown-char"),
        }
    }
}
