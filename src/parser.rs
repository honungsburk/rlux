use std::collections::binary_heap::Iter;

use crate::{
    expression::{self, Expr},
    position::*,
    token::{Token, TokenType},
};

/// Production rules:
///
/// ```bnf
/// expression     → equality ;
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" ) unary
///                | primary ;
/// primary        → NUMBER | STRING | "true" | "false" | "nil"
///                | "(" expression ")" ;
/// ```
pub struct Parser<'a> {
    current: usize,
    tokens: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            current: 0,
            tokens: tokens,
        }
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current];
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1];
    }

    fn is_at_end(&self) -> bool {
        return self.peek() == Token::Eof;
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            token == self.peek().into()
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn one_of<T: IntoIterator<Item = TokenType>>(&mut self, tokens: T) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn is(&mut self, token: TokenType) -> bool {
        if self.check(token.into()) {
            self.advance();
            return true;
        }
        return false;
    }
}

pub enum ParserError {}

pub fn run(tokens: &Vec<Token>) -> Result<Expr, Vec<ParserError>> {
    let parser = Parser::new(tokens);
}

fn comparison(p: &mut Parser) -> Expr {
    let mut expr = term(p);

    while p.one_of(vec![
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::Less,
        TokenType::LessEqual,
    ]) {
        let operator = p.previous();
        let right = term(p);
        expr = Expr::binary(expr, operator, right);
    }

    expr
}

fn term(p: &mut Parser) -> Expr {
    let mut expr: Expr = factor(p);

    while p.one_of(vec![TokenType::Minus, TokenType::Plus]) {
        let operator = p.previous();
        let right = factor(p);
        expr = Expr::binary(expr, operator, right);
    }

    expr
}

fn factor(p: &mut Parser) -> Expr {
    let mut expr: Expr = unary(p);

    while p.one_of(vec![TokenType::Slash, TokenType::Star]) {
        let operator = p.previous();
        let right = unary(p);
        expr = Expr::binary(expr, operator, right);
    }

    expr
}

fn unary(p: &mut Parser) -> Expr {
    if p.one_of(vec![TokenType::Bang, TokenType::Minus]) {
        let operator = p.previous();
        let right = unary(p);
        return Expr::unary(operator, right);
    }

    primary(p)
}

fn primary(p: &mut Parser) -> Expr {
    if p.is(TokenType::False) {
        return Expr::false_expr();
    }

    if p.is(TokenType::True) {
        return Expr::true_expr();
    }
    if p.is(TokenType::Nil) {
        return Expr::nil();
    }

    if let Token::Number(n) = p.peek() {
        p.advance();
        return Expr::number(n);
    }

    if let Token::String(s) = p.peek() {
        p.advance();
        return Expr::string(s);
    }

    if p.is(TokenType::LeftParen) {
        let expr = expression(p);
        p.consume(TokenType::RightParen);
        return Expr::grouping(expr);
    }
}
