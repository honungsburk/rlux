use crate::{
    expr::{BinaryOp, Expr, UnaryOp},
    parser::Parser,
    position::{Diagnostic, WithSpan},
    token::{Token, TokenKind},
};

/// Parse the given tokens into an expression.
pub fn run(tokens: &Vec<WithSpan<Token>>) -> Result<Expr, Vec<Diagnostic>> {
    let mut parser = Parser::new(tokens);

    match expression(&mut parser) {
        Some(expr) => Ok(expr),
        None => Err(Vec::from_iter(
            parser.diagnostics().iter().map(|d| d.clone()),
        )),
    }
}

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
pub fn expression(p: &mut Parser) -> Option<Expr> {
    equality(p)
}

fn equality(p: &mut Parser) -> Option<Expr> {
    let mut expr = comparison(p)?;

    while p.one_of(vec![TokenKind::BangEqual, TokenKind::EqualEqual]) {
        let operator = match p.previous().as_ref().value {
            Token::BangEqual => BinaryOp::NotEquals,
            Token::EqualEqual => BinaryOp::Equals,
            op => panic!("Matched a binary operator that doesn't exist: {}", op),
        };
        let right = comparison(p)?;
        expr = Expr::binary(expr, operator, right);
    }

    Some(expr)
}

fn comparison(p: &mut Parser) -> Option<Expr> {
    let mut expr = term(p)?;

    while p.one_of(vec![
        TokenKind::Greater,
        TokenKind::GreaterEqual,
        TokenKind::Less,
        TokenKind::LessEqual,
    ]) {
        let operator = match p.previous().as_ref().value {
            Token::Greater => BinaryOp::Greater,
            Token::GreaterEqual => BinaryOp::GreaterOrEquals,
            Token::Less => BinaryOp::Less,
            Token::LessEqual => BinaryOp::LessOrEquals,
            op => panic!("Matched a binary operator that doesn't exist: {}", op),
        };
        let right = term(p)?;
        expr = Expr::binary(expr, operator, right);
    }

    Some(expr)
}

fn term(p: &mut Parser) -> Option<Expr> {
    let mut expr: Expr = factor(p)?;

    while p.one_of(vec![TokenKind::Minus, TokenKind::Plus]) {
        let operator = match p.previous().as_ref().value {
            Token::Minus => BinaryOp::Minus,
            Token::Plus => BinaryOp::Plus,
            op => panic!("Matched a binary operator that doesn't exist: {}", op),
        };
        let right = factor(p)?;
        expr = Expr::binary(expr, operator, right);
    }

    Some(expr)
}

fn factor(p: &mut Parser) -> Option<Expr> {
    let mut expr: Expr = unary(p)?;

    while p.one_of(vec![TokenKind::Slash, TokenKind::Star]) {
        let operator = match p.previous().as_ref().value {
            Token::Slash => BinaryOp::Divide,
            Token::Star => BinaryOp::Multiply,
            op => panic!("Matched a binary operator that doesn't exist: {}", op),
        };
        let right = unary(p)?;
        expr = Expr::binary(expr, operator, right);
    }

    Some(expr)
}

fn unary(p: &mut Parser) -> Option<Expr> {
    if p.one_of(vec![TokenKind::Bang, TokenKind::Minus]) {
        let operator = match p.previous().as_ref().value {
            Token::Bang => UnaryOp::Not,
            Token::Minus => UnaryOp::Negate,
            op => panic!("Matched a uniary operator that doesn't exist: {}", op),
        };
        let right = unary(p)?;
        return Some(Expr::unary(operator, right));
    }

    primary(p)
}

fn primary(p: &mut Parser) -> Option<Expr> {
    if p.is(TokenKind::False) {
        return Some(Expr::false_expr());
    }

    if p.is(TokenKind::True) {
        return Some(Expr::true_expr());
    }
    if p.is(TokenKind::Nil) {
        return Some(Expr::nil());
    }

    if let Token::Number(n) = p.peek_token().value {
        p.advance();
        return Some(Expr::number(n));
    }

    if let Token::String(s) = p.peek_token().value.clone() {
        p.advance();
        return Some(Expr::string(s));
    }

    if p.is(TokenKind::LeftParen) {
        let expr = expression(p)?;
        return p
            .expect(TokenKind::RightParen)
            .map(|_| Expr::grouping(expr));
    }

    let token = p.peek_token();

    p.error(
        &format!(
            "Expected one of true, false, nil, number, string, or ( but found {}",
            token.value
        ),
        token.span,
    );

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        expr::{BinaryOp, Expr},
        token::Token,
    };

    fn token(kind: Token) -> WithSpan<Token> {
        WithSpan::new_unchecked(kind, 0, 1)
    }

    fn run_test(tokens: &Vec<Token>) -> Result<Expr, Vec<Diagnostic>> {
        let tokens = tokens.into_iter().map(|t| token(t.clone())).collect();
        run(&tokens)
    }

    #[test]
    fn test_precedence() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Star,
            Token::Number(3.0),
            Token::Eof,
        ];

        let expr = run_test(&tokens).unwrap();
        assert_eq!(
            expr,
            Expr::binary(
                Expr::number(1.0),
                BinaryOp::Plus,
                Expr::binary(Expr::number(2.0), BinaryOp::Multiply, Expr::number(3.0)),
            )
        );
    }

    #[test]
    fn test_parens() {
        let tokens = vec![
            Token::LeftParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RightParen,
            Token::Star,
            Token::Number(3.0),
            Token::Eof,
        ];

        let expr = run_test(&tokens).unwrap();
        assert_eq!(
            expr,
            Expr::binary(
                Expr::grouping(Expr::binary(
                    Expr::number(1.0),
                    BinaryOp::Plus,
                    Expr::number(2.0)
                )),
                BinaryOp::Multiply,
                Expr::number(3.0),
            )
        );
    }

    #[test]
    fn test_parser_error() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Star,
            Token::Eof,
        ];

        let diagnostics = run_test(&tokens).unwrap_err();
        assert_eq!(diagnostics.len(), 1);
    }
}
