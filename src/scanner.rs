use std::{borrow::Borrow, iter::Peekable, str::Chars};

use crate::position::*;
use crate::token::Token;

pub struct Scanner<'a> {
    start: BytePos,
    current: BytePos,
    source: &'a str,
    it: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(buf: &str) -> Scanner {
        Scanner {
            current: BytePos::default(),
            start: BytePos::default(),
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

    pub fn run(&mut self) -> Vec<WithSpan<Token>> {
        let mut tokens = Vec::new();
        while let Some(c) = self.next() {
            // let initial_position = self.current_position;
            if let Some(token) = self.scan_token(c) {
                tokens.push(self.with_span(token));
            }
            self.start = self.current;
        }
        tokens
    }

    fn scan_token(&mut self, c: char) -> Option<Token> {
        match c {
            // Single-character tokens
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '-' => Some(Token::Minus),
            '+' => Some(Token::Plus),
            ';' => Some(Token::Semicolon),
            '*' => Some(Token::Star),
            // Two-character tokens
            '!' => Some(self.either('=', Token::BangEqual, Token::Bang)),
            '=' => Some(self.either('=', Token::EqualEqual, Token::Equal)),
            '<' => Some(self.either('=', Token::LessEqual, Token::Less)),
            '>' => Some(self.either('=', Token::GreaterEqual, Token::Greater)),
            '/' => {
                if self.next_match('/') {
                    while self.peek() != Some(&'\n') && self.peek().is_some() {
                        self.next();
                    }
                    None
                } else {
                    Some(Token::Slash)
                }
            }
            ' ' | '\r' | '\t' => None,
            '\n' => None,
            '"' => Some(self.string()),
            _ if c.is_ascii_digit() => Some(self.number(c)),
            // kewwords are reserved identifiers!
            _ if c.is_ascii_alphabetic() || c == '_' => Some(fix_keywords(self.identifier(c))),
            c => Some(Token::UnknownChar(c)),
        }
    }

    // Consume next char if it matches
    fn consume_if<F>(&mut self, x: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(&ch) = self.peek() {
            if x(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn either(&mut self, to_match: char, matched: Token, unmatched: Token) -> Token {
        if self.consume_if(|ch| ch == to_match) {
            matched
        } else {
            unmatched
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

    fn with_span(&self, token_type: Token) -> WithSpan<Token> {
        WithSpan::new_unchecked(token_type, self.start.0, self.current.0)
    }

    fn string(&mut self) -> Token {
        while self.peek() != Some(&'"') && self.peek().is_some() {
            self.next();
        }

        if self.peek().is_none() {
            return Token::UnterminatedString;
        }

        // Consume the closing "
        self.next();
        Token::String(self.source[self.start.0 + 1..self.current.0 - 1].to_string())
    }

    fn number(&mut self, first: char) -> Token {
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

        Token::Number(self.source[self.start.0..self.current.0].parse().unwrap())
    }

    fn identifier(&mut self, first: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first);

        self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_')
            .iter()
            .for_each(|c| identifier.push(*c));

        Token::Identifier(identifier)
    }
}

fn fix_keywords(mut token: Token) -> Token {
    match token {
        Token::Identifier(s) => {
            return match s.borrow() {
                "and" => Token::And,
                "or" => Token::Or,
                "false" => Token::False,
                "true" => Token::True,
                "if" => Token::If,
                "else" => Token::Else,
                "class" => Token::Class,
                "for" => Token::For,
                "while" => Token::While,
                "fun" => Token::Fun,
                "nil" => Token::Nil,
                "print" => Token::Print,
                "return" => Token::Return,
                "super" => Token::Super,
                "this" => Token::This,
                "var" => Token::Var,
                _ => Token::Identifier(s),
            };
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
            WithSpan::new_unchecked(Token::LeftParen, 0, 1),
            WithSpan::new_unchecked(Token::RightParen, 1, 2),
            WithSpan::new_unchecked(Token::LeftBrace, 2, 3),
            WithSpan::new_unchecked(Token::RightBrace, 3, 4),
            WithSpan::new_unchecked(Token::Comma, 4, 5),
            WithSpan::new_unchecked(Token::Dot, 5, 6),
            WithSpan::new_unchecked(Token::Minus, 6, 7),
            WithSpan::new_unchecked(Token::Plus, 7, 8),
            WithSpan::new_unchecked(Token::Semicolon, 8, 9),
            WithSpan::new_unchecked(Token::Star, 9, 10),
            WithSpan::new_unchecked(Token::Slash, 10, 11),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_two_char_tokens() {
        let mut scanner = Scanner::new("!= == <= >=");
        let tokens = scanner.run();
        let expected = vec![
            WithSpan::new_unchecked(Token::BangEqual, 0, 2),
            WithSpan::new_unchecked(Token::EqualEqual, 3, 5),
            WithSpan::new_unchecked(Token::LessEqual, 6, 8),
            WithSpan::new_unchecked(Token::GreaterEqual, 9, 11),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comments() {
        let mut scanner = Scanner::new("!= // == <= >=");
        let tokens = scanner.run();
        let expected = vec![WithSpan::new_unchecked(Token::BangEqual, 0, 2)];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_logical_operators() {
        let mut scanner = Scanner::new("false or (false and true)");
        let tokens = scanner.run();
        let expected = vec![
            WithSpan::new_unchecked(Token::False, 0, 5),
            WithSpan::new_unchecked(Token::Or, 6, 8),
            WithSpan::new_unchecked(Token::LeftParen, 9, 10),
            WithSpan::new_unchecked(Token::False, 10, 15),
            WithSpan::new_unchecked(Token::And, 16, 19),
            WithSpan::new_unchecked(Token::True, 20, 24),
            WithSpan::new_unchecked(Token::RightParen, 24, 25),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_string() {
        let mut scanner = Scanner::new("\"Hello, world!\"");
        let tokens = scanner.run();
        let expected = vec![WithSpan::new_unchecked(
            Token::String("Hello, world!".to_string()),
            0,
            15,
        )];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_number_with_dot() {
        let mut scanner = Scanner::new("123.45");
        let tokens = scanner.run();
        let expected = vec![WithSpan::new_unchecked(Token::Number(123.45), 0, 6)];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_number_without_dot() {
        let mut scanner = Scanner::new("123");
        let tokens = scanner.run();
        let expected = vec![WithSpan::new_unchecked(Token::Number(123.0), 0, 3)];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_identifier() {
        let mut scanner = Scanner::new("identifier");
        let tokens = scanner.run();
        let expected = vec![WithSpan::new_unchecked(
            Token::Identifier("identifier".to_string()),
            0,
            10,
        )];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_keywords() {
        let mut scanner = Scanner::new(
            "and or false true if else class for while fun nil print return super this var",
        );
        let tokens = scanner.run();
        let expected = vec![
            WithSpan::new_unchecked(Token::And, 0, 3),
            WithSpan::new_unchecked(Token::Or, 4, 6),
            WithSpan::new_unchecked(Token::False, 7, 12),
            WithSpan::new_unchecked(Token::True, 13, 17),
            WithSpan::new_unchecked(Token::If, 18, 20),
            WithSpan::new_unchecked(Token::Else, 21, 25),
            WithSpan::new_unchecked(Token::Class, 26, 31),
            WithSpan::new_unchecked(Token::For, 32, 35),
            WithSpan::new_unchecked(Token::While, 36, 41),
            WithSpan::new_unchecked(Token::Fun, 42, 45),
            WithSpan::new_unchecked(Token::Nil, 46, 49),
            WithSpan::new_unchecked(Token::Print, 50, 55),
            WithSpan::new_unchecked(Token::Return, 56, 62),
            WithSpan::new_unchecked(Token::Super, 63, 68),
            WithSpan::new_unchecked(Token::This, 69, 73),
            WithSpan::new_unchecked(Token::Var, 74, 77),
        ];

        assert_eq!(tokens, expected);
    }
}
