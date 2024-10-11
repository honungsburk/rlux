use std::{borrow::Borrow, fmt::Display, iter::Peekable, str::Chars};

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
    Identifier(String),
    String(String),
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
            TokenType::Identifier(s) => write!(f, "{}", s),
            TokenType::String(s) => write!(f, "\"{}\"", s),
            TokenType::Number(n) => write!(f, "{}", n),
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
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
    pub fn from_type(token_type: TokenType) -> Token {
        let lexeme = token_type.to_string();
        Token {
            token_type,
            lexeme: lexeme,
            line: 1,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct BytePos(pub usize);

impl BytePos {
    pub fn shift(self, ch: char) -> Self {
        BytePos(self.0 + ch.len_utf8())
    }
}

pub struct Scanner<'a> {
    start: BytePos,
    current: BytePos,
    line: u32,
    source: &'a str,
    it: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(buf: &str) -> Scanner {
        Scanner {
            current: BytePos::default(),
            start: BytePos::default(),
            line: 1,
            source: buf,
            it: buf.chars().peekable(),
        }
    }

    fn next(&mut self) -> Option<char> {
        let next = self.it.next();
        if let Some(c) = next {
            self.current = self.current.shift(c);
        }
        next
    }

    fn peek(&mut self) -> Option<&char> {
        self.it.peek()
    }

    fn consume_if<P>(&mut self, predicate: P) -> bool
    where
        P: Fn(char) -> bool,
    {
        if let Some(&c) = self.peek() {
            if predicate(c) {
                self.next().unwrap(); // To trigger errors if there are any
                return true;
            }
        }
        false
    }

    // Consume next char if the next one after matches (so .3 eats . if 3 is numeric, for example)
    fn consume_if_next<P>(&mut self, predicate: P) -> bool
    where
        P: Fn(char) -> bool,
    {
        let mut it = self.it.clone();

        match it.next() {
            Some(_) => {
                if let Some(c) = it.peek() {
                    if predicate(*c) {
                        self.next().unwrap();
                        return true;
                    }
                }
                return false;
            }
            None => return false,
        }
    }

    fn consume_while<P>(&mut self, predicate: P) -> Vec<char>
    where
        P: Fn(char) -> bool,
    {
        let mut consumed = Vec::new();
        while let Some(&c) = self.peek() {
            if predicate(c) {
                consumed.push(c);
                self.next().unwrap();
            } else {
                break;
            }
        }
        consumed
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.next() {
            // let initial_position = self.current_position;
            if let Some(token) = self.scan_token(c) {
                tokens.push(token);
            }
            self.start = self.current;
        }
        tokens
    }

    fn scan_token(&mut self, c: char) -> Option<Token> {
        match c {
            // Single-character tokens
            '(' => self.create_token(TokenType::LeftParen),
            ')' => self.create_token(TokenType::RightParen),
            '{' => self.create_token(TokenType::LeftBrace),
            '}' => self.create_token(TokenType::RightBrace),
            ',' => self.create_token(TokenType::Comma),
            '.' => self.create_token(TokenType::Dot),
            '-' => self.create_token(TokenType::Minus),
            '+' => self.create_token(TokenType::Plus),
            ';' => self.create_token(TokenType::Semicolon),
            '*' => self.create_token(TokenType::Star),
            // Two-character tokens
            '!' => {
                if self.next_match('=') {
                    self.create_token(TokenType::BangEqual)
                } else {
                    self.create_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.next_match('=') {
                    self.create_token(TokenType::EqualEqual)
                } else {
                    self.create_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.next_match('=') {
                    self.create_token(TokenType::LessEqual)
                } else {
                    self.create_token(TokenType::Less)
                }
            }
            '>' => {
                if self.next_match('=') {
                    self.create_token(TokenType::GreaterEqual)
                } else {
                    self.create_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.next_match('/') {
                    while self.peek() != Some(&'\n') && self.peek().is_some() {
                        self.next();
                    }
                    None
                } else {
                    self.create_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            '"' => self.string(),
            _ if c.is_ascii_digit() => self.number(c),
            // kewwords are reserved identifiers!
            _ if c.is_ascii_alphabetic() || c == '_' => self.identifier(c).map(fix_keywords),
            _ => {
                crate::error(self.line, "Unexpected character.");
                None
            }
        }
    }

    fn next_match(&mut self, expected: char) -> bool {
        if self.peek() == Some(&expected) {
            self.next();
            true
        } else {
            false
        }
    }

    fn create_token(&self, token_type: TokenType) -> Option<Token> {
        let text = self.source[self.start.0..self.current.0].to_string();
        Some(Token {
            token_type,
            lexeme: text,
            line: self.line,
        })
    }

    fn string(&mut self) -> Option<Token> {
        while self.peek() != Some(&'"') && self.peek().is_some() {
            if self.peek() == Some(&'\n') {
                self.line += 1;
            }
            self.next();
        }

        if self.peek().is_none() {
            crate::error(self.line, "Unterminated string.");
            return None;
        }

        // Consume the closing "
        self.next();
        self.create_token(TokenType::String(
            self.source[self.start.0 + 1..self.current.0 - 1].to_string(),
        ))
    }

    fn number(&mut self, first: char) -> Option<Token> {
        let mut number = String::new();
        number.push(first);

        self.consume_while(|c| c.is_ascii_digit())
            .iter()
            .for_each(|c| number.push(*c));

        if self.peek() == Some(&'.') && self.consume_if_next(|c| c.is_ascii_digit()) {
            number.push('.');
            self.consume_while(|c| c.is_ascii_digit())
                .iter()
                .for_each(|c| number.push(*c));
        }

        self.create_token(TokenType::Number(
            self.source[self.start.0..self.current.0].parse().unwrap(),
        ))
    }

    fn identifier(&mut self, first: char) -> Option<Token> {
        let mut identifier = String::new();
        identifier.push(first);

        self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_')
            .iter()
            .for_each(|c| identifier.push(*c));

        self.create_token(TokenType::Identifier(identifier))
    }
}

fn fix_keywords(mut token: Token) -> Token {
    match token.token_type {
        TokenType::Identifier(s) => {
            let new_token_type = match s.borrow() {
                "and" => TokenType::And,
                "or" => TokenType::Or,
                "false" => TokenType::False,
                "true" => TokenType::True,
                "if" => TokenType::If,
                "else" => TokenType::Else,
                "class" => TokenType::Class,
                "for" => TokenType::For,
                "while" => TokenType::While,
                "fun" => TokenType::Fun,
                "nil" => TokenType::Nil,
                "print" => TokenType::Print,
                "return" => TokenType::Return,
                "super" => TokenType::Super,
                "this" => TokenType::This,
                "var" => TokenType::Var,
                _ => TokenType::Identifier(s),
            };
            token.token_type = new_token_type;
            token
        }
        _ => token,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_char_tokens() {
        let mut scanner = Scanner::new("(){},.-+;*/");
        let tokens = scanner.run();
        let expected = vec![
            Token::from_type(TokenType::LeftParen),
            Token::from_type(TokenType::RightParen),
            Token::from_type(TokenType::LeftBrace),
            Token::from_type(TokenType::RightBrace),
            Token::from_type(TokenType::Comma),
            Token::from_type(TokenType::Dot),
            Token::from_type(TokenType::Minus),
            Token::from_type(TokenType::Plus),
            Token::from_type(TokenType::Semicolon),
            Token::from_type(TokenType::Star),
            Token::from_type(TokenType::Slash),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_two_char_tokens() {
        let mut scanner = Scanner::new("!= == <= >=");
        let tokens = scanner.run();
        let expected = vec![
            Token::from_type(TokenType::BangEqual),
            Token::from_type(TokenType::EqualEqual),
            Token::from_type(TokenType::LessEqual),
            Token::from_type(TokenType::GreaterEqual),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comments() {
        let mut scanner = Scanner::new("!= // == <= >=");
        let tokens = scanner.run();
        let expected = vec![Token::from_type(TokenType::BangEqual)];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_logical_operators() {
        let mut scanner = Scanner::new("false or (false and true)");
        let tokens = scanner.run();
        let expected = vec![
            Token::from_type(TokenType::False),
            Token::from_type(TokenType::Or),
            Token::from_type(TokenType::LeftParen),
            Token::from_type(TokenType::False),
            Token::from_type(TokenType::And),
            Token::from_type(TokenType::True),
            Token::from_type(TokenType::RightParen),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string() {
        let mut scanner = Scanner::new("\"Hello, world!\"");
        let tokens = scanner.run();
        let expected = vec![Token::from_type(TokenType::String(
            "Hello, world!".to_string(),
        ))];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_number_with_dot() {
        let mut scanner = Scanner::new("123.45");
        let tokens = scanner.run();
        let expected = vec![Token::from_type(TokenType::Number(123.45))];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_number_without_dot() {
        let mut scanner = Scanner::new("123");
        let tokens = scanner.run();
        let expected = vec![Token::from_type(TokenType::Number(123.0))];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_identifier() {
        let mut scanner = Scanner::new("identifier");
        let tokens = scanner.run();
        let expected = vec![Token::from_type(TokenType::Identifier(
            "identifier".to_string(),
        ))];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_keywords() {
        let mut scanner = Scanner::new(
            "and or false true if else class for while fun nil print return super this var",
        );
        let tokens = scanner.run();
        let expected = vec![
            Token::from_type(TokenType::And),
            Token::from_type(TokenType::Or),
            Token::from_type(TokenType::False),
            Token::from_type(TokenType::True),
            Token::from_type(TokenType::If),
            Token::from_type(TokenType::Else),
            Token::from_type(TokenType::Class),
            Token::from_type(TokenType::For),
            Token::from_type(TokenType::While),
            Token::from_type(TokenType::Fun),
            Token::from_type(TokenType::Nil),
            Token::from_type(TokenType::Print),
            Token::from_type(TokenType::Return),
            Token::from_type(TokenType::Super),
            Token::from_type(TokenType::This),
            Token::from_type(TokenType::Var),
        ];

        assert_eq!(tokens, expected);
    }
}
