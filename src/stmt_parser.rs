use crate::{
 ast::{Expr, Stmt}, expr_parser::expression, interpreter::LuxValue, parser::Parser, token::{Token, TokenKind}
};


pub fn declaration(p: &mut Parser) -> Option<Stmt> {
    if p.is(TokenKind::Fun){
        return function(p);
    } else if p.is(TokenKind::Var) {
        let name = p.expect(TokenKind::Identifier)?;
        let expr = if p.check(TokenKind::Equal) {
            p.expect(TokenKind::Equal)?;
            expression(p)?
        } else {
            Expr::nil()
        };
        p.expect(TokenKind::Semicolon)?;
        match &name.value {
            Token::Identifier(name) => return Some(Stmt::Var(name.clone(), expr)),
            _ => panic!("Expected identifier"),
        }
    }
    statement(p)
}

fn function(p: &mut Parser) -> Option<Stmt> {

    // signature

    let name = p.expect(TokenKind::Identifier)?;


    let name = if let Token::Identifier(id) = name.value.clone() {
        id
    } else {
        panic!("Expected an indentifer but it wasn't")
    };

    p.expect(TokenKind::LeftParen)?;
    let mut parameters = Vec::new();

    if !p.check(TokenKind::RightParen) {
        loop {
            if parameters.len() > 255 {
                p.error("Can't have more than 255 parameters.", p.peek_token().span);
            }
            let par = p.expect(TokenKind::Identifier)?;

            if let Token::Identifier(id) = par.value.clone() {
                parameters.push(id);
            } else {
                panic!("Expected an indentifer but it wasn't")
            }

            if !p.is(TokenKind::Comma){
                break;
            }
        }
    }
    p.expect(TokenKind::RightParen);

    // body

    let body = block(p)?;

    Some(Stmt::Function(name, parameters, Box::new(body)))
}



fn statement(p: &mut Parser) -> Option<Stmt> {
    if p.check(TokenKind::For) {
        return for_statement(p);
    } else if p.check(TokenKind::If) {
        return if_statement(p);
    } else if p.is(TokenKind::Print) {
        let expr = expression(p)?;
        p.expect(TokenKind::Semicolon)?;
        return Some(Stmt::Print(expr));
    } else if p.is(TokenKind::Return){
        if p.is(TokenKind::Semicolon) {
            return Some(Stmt::Return(Expr::Nil))
        } else {
            let expr = expression(p)?;
            p.expect(TokenKind::Semicolon)?;
            return Some(Stmt::Return(expr))
        }
    } else if p.check(TokenKind::While) {
        return while_statement(p);
    }else if p.check(TokenKind::LeftBrace) {
        return block(p);
    } else {
        let expr = expression(p)?;
        p.expect(TokenKind::Semicolon)?;
        return Some(Stmt::Expression(expr));
    }
}




fn for_statement(p: &mut Parser) -> Option<Stmt> {
    p.expect(TokenKind::For)?;
    p.expect(TokenKind::LeftParen)?;

    let initializer: Option<Stmt> = if p.is(TokenKind::Semicolon) {
        None
    } else if p.check(TokenKind::Var) {
        Some(declaration(p)?)
    } else {
        let expr = expression(p)?;
        p.expect(TokenKind::Semicolon)?;
        Some(Stmt::Expression(expr))
    };

    let condition = if p.check(TokenKind::Semicolon) {
        Expr::true_expr()
    } else {
        expression(p)?
    };

    p.expect(TokenKind::Semicolon)?;

    let increment = if p.check(TokenKind::RightParen) {
        None
    } else {
        Some(expression(p)?)
    };

    p.expect(TokenKind::RightParen)?;
    let body = statement(p)?;

    // Construct the for loop as a while loop
    let while_body = if let Some(inc) = increment {
        Stmt::Block(vec![body, Stmt::Expression(inc)])
    } else {
        body
    };

    let while_stmt = Stmt::While(condition, Box::new(while_body));

    let for_stmt = if let Some(init) = initializer {
        Stmt::Block(vec![init, while_stmt])
    } else {
        while_stmt
    };

    Some(for_stmt)
}

fn while_statement(p: &mut Parser) -> Option<Stmt> {
    p.expect(TokenKind::While)?;
    p.expect(TokenKind::LeftParen)?;
    let cond = expression(p)?;
    p.expect(TokenKind::RightParen)?;
    let body = statement(p)?;
    Some(Stmt::While(cond, Box::new(body)))
}

fn if_statement(p: &mut Parser) -> Option<Stmt> {
    p.expect(TokenKind::If)?;
    p.expect(TokenKind::LeftParen)?;
    let cond = expression(p)?;
    p.expect(TokenKind::RightParen)?;
    let then = statement(p)?;
    if p.is(TokenKind::Else) { 
        let stmt = statement(p)?;
        Some(Stmt::If(cond, Box::new(then), Some(Box::new(stmt))))
    } else { 
        Some(Stmt::If(cond, Box::new(then), None))
    }
}


fn block(p: &mut Parser) -> Option<Stmt> {
    let mut stmts = Vec::new();
    p.expect(TokenKind::LeftBrace)?;
    while !p.check(TokenKind::RightBrace) && !p.is_at_end() {
        let stmt = declaration(p)?;
        stmts.push(stmt);
    }

    p.expect(TokenKind::RightBrace)?;
    Some(Stmt::Block(stmts))
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
    use crate::{ast::Expr, position::{Diagnostic, WithSpan}, token::Token};

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


    #[test]
    fn test_can_parse_empty_block() {
        let tokens = vec![Token::LeftBrace, Token::RightBrace];
        let stmt = run_test(&tokens);
        assert_eq!(stmt, Ok(Stmt::Block(Vec::new())));
    }

    #[test]
    fn test_can_parse_block_with_statements() {
        let tokens = vec![Token::LeftBrace, Token::Number(1.0), Token::Semicolon, Token::RightBrace];
        let stmt = run_test(&tokens);
        assert_eq!(stmt, Ok(Stmt::Block(vec![Stmt::Expression(Expr::Number(1.0))])));
    }

    #[test]
    fn test_can_parse_block_with_nested_blocks() {
        let tokens = vec![Token::LeftBrace, Token::LeftBrace, Token::RightBrace, Token::RightBrace];
        let stmt = run_test(&tokens);
        assert_eq!(stmt, Ok(Stmt::Block(vec![Stmt::Block(Vec::new())])));
    }

}