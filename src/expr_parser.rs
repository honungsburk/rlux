use crate::{
    ast::expr::{BinaryOp, Expr, UnaryOp},
    parser::Parser,
    token::{Token, TokenKind},
};



/// Production rules:
///
/// ```bnf
/// expression     → assignment ;
///
/// assignment     → IDENTIFIER "=" expression 
///               | logical_or ;
/// 
/// logical_or     → logical_and ( "or" logical_and )* ;
/// logical_and    → equality ( "and" equality )* ;
/// 
/// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
/// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
/// term           → factor ( ( "-" | "+" ) factor )* ;
/// factor         → unary ( ( "/" | "*" ) unary )* ;
/// unary          → ( "!" | "-" ) unary | call ;
/// call           → primary ( "(" arguments? ")" )* ;
/// primary        → NUMBER | STRING | "true" | "false" | "nil"
///                | "(" expression ")" | IDENTIFIER ;
/// ```
pub fn expression(p: &mut Parser) -> Option<Expr> {
    assignment(p)
}

fn assignment(p: &mut Parser) -> Option<Expr> {
    let expr = logical_or(p)?;
    if p.is(TokenKind::Equal) {
        let previous = p.previous();
        let value = assignment(p)?;
        match &expr {
            Expr::Variable(name) => return Some(Expr::assignment(name.clone(), value)),
            _ => {
                p.error("Invalid assignment target", previous.span);
                return None;
            },
        }
    }
    Some(expr)
}

fn logical_or(p: &mut Parser) -> Option<Expr> {
    let mut expr = logical_and(p)?;
    while p.is(TokenKind::Or) {
        let right = logical_and(p)?;
        expr = Expr::logical_or(expr, right);
    }
    Some(expr)
}

fn logical_and(p: &mut Parser) -> Option<Expr> {
    let mut expr = equality(p)?;
    while p.is(TokenKind::And) {
        let right = equality(p)?;
        expr = Expr::logical_and(expr, right);
    }
    Some(expr)
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

    calls(p)
}

fn calls(p: &mut Parser) -> Option<Expr> {
    let mut expr = primary(p)?;

    while p.check(TokenKind::LeftParen) {
        expr = call(p, expr)?;
    }

    Some(expr)
}

fn call(p: &mut Parser, callee: Expr) -> Option<Expr> {
    let mut arguments = vec![];

    p.expect(TokenKind::LeftParen)?;

    if !p.check(TokenKind::RightParen) {
        loop {                                                                      
            if arguments.len() > 255 {
                return None;
            }

            let expr = expression(p)?;
            arguments.push(expr);

            if !p.is(TokenKind::Comma) {
                break;
            }
        } 
    }

    p.expect(TokenKind::RightParen)?;

    Some(Expr::call(callee, arguments))

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

    if p.is(TokenKind::Identifier) {
        let token = p.previous();
        match &token.value {
            Token::Identifier(name) => return Some(Expr::variable(name.clone())),
            _ => panic!("Expected identifier"),
        }
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
        ast::expr::{BinaryOp, Expr},
        token::Token,
        position::{Diagnostic, WithSpan},
    };

    fn token(kind: Token) -> WithSpan<Token> {
        WithSpan::new_unchecked(kind, 0, 1)
    }


    /// Parse the given tokens into an expression.
    fn run_test(tokens: &Vec<Token>) -> Result<Expr, Vec<Diagnostic>> {
        let tokens: Vec<WithSpan<Token>> = tokens.into_iter().map(|t| token(t.clone())).collect();
        let mut parser = Parser::new(&tokens);

        match expression(&mut parser) {
            Some(expr) => Ok(expr),
            None => Err(Vec::from_iter(
                parser.diagnostics().iter().map(|d| d.clone()),
            )),
        }
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
