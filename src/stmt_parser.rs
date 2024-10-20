use crate::{
 expr_parser::expression, parser::Parser, stmt::Stmt, token::{Token, TokenKind}
};


pub fn declaration(p: &mut Parser) -> Option<Stmt> {
    if p.is(TokenKind::Var) {
        let name = p.expect(TokenKind::Identifier)?;
        p.expect(TokenKind::Equal)?;
        let expr = expression(p)?;
        p.expect(TokenKind::Semicolon)?;
        match &name.value {
            Token::Identifier(name) => return Some(Stmt::Var(name.clone(), expr)),
            _ => panic!("Expected identifier"),
        }
    }
    statement(p)
}


pub fn statement(p: &mut Parser) -> Option<Stmt> {
    if p.is(TokenKind::Print) {
        let expr = expression(p)?;
        p.expect(TokenKind::Semicolon)?;
        return Some(Stmt::Print(expr));
    } else {
        let expr = expression(p)?;
        p.expect(TokenKind::Semicolon)?;
        return Some(Stmt::Expression(expr));
    }
}

/// Drop tokens until a statement is found or the end of the file is reached.
///
/// This is used to drop tokens after an error occurs and put the parser back in a valid state.
pub fn drop_until_statement(p: &mut Parser) {
    while !p.is_at_end() && !p.is(TokenKind::Semicolon) {
        p.advance();
    }

    if !p.is_at_end() {
        p.expect(TokenKind::Semicolon);
    }
}


#[cfg(test)]
mod tests {
    use crate::{expr::Expr, position::{Diagnostic, WithSpan}, token::Token};

    use super::*;

    fn token(kind: Token) -> WithSpan<Token> {
        WithSpan::new_unchecked(kind, 0, 1)
    }


    /// Parse the given tokens into an expression.
    fn run_test(tokens: &Vec<Token>) -> Result<Stmt, Vec<Diagnostic>> {
        let tokens: Vec<WithSpan<Token>> = tokens.into_iter().map(|t| token(t.clone())).collect();
        let mut parser = Parser::new(&tokens);

        match statement(&mut parser) {
            Some(expr) => Ok(expr),
            None => Err(Vec::from_iter(
                parser.diagnostics().iter().map(|d| d.clone()),
            )),
        }
    }


    #[test]
    fn test_can_parse_print_statement() {
        let tokens = vec![Token::Print, Token::Number(1.0), Token::Semicolon];
        let stmt = run_test(&tokens);
        assert_eq!(stmt, Ok(Stmt::Print(Expr::Number(1.0))));
    }

    #[test]
    fn test_can_parse_expression_statement() {
        let tokens = vec![Token::Number(1.0), Token::Semicolon];
        let stmt = run_test(&tokens);
        assert_eq!(stmt, Ok(Stmt::Expression(Expr::Number(1.0))));
    }

    #[test]
    fn test_cannot_parse_invalid_statement() {
        let tokens = vec![Token::Number(1.0)];
        let stmt = run_test(&tokens);
        assert!(stmt.is_err());
    }
}
