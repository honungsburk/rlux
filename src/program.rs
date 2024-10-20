use crate::{
    environment::Environment,
    parser::Parser,
    position::{Diagnostic, WithSpan},
    run_time_error::RunTimeError,
    stmt::Stmt,
    stmt_eval,
    stmt_parser::{declaration, drop_until_statement},
    token::Token,
};

pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn parse(tokens: &Vec<WithSpan<Token>>) -> Result<Self, Vec<Diagnostic>> {
        let mut parser = Parser::new(tokens);
        let mut statements = Vec::new();

        while !parser.is_at_end() {
            if let Some(stmt) = declaration(&mut parser) {
                statements.push(stmt);
            } else {
                // We want to find all statements after the error occurs.
                // So we drop tokens to get the parser back in a valid state.
                drop_until_statement(&mut parser);
            }
        }

        if parser.had_error() {
            return Err(Vec::from_iter(
                parser.diagnostics().iter().map(|d| d.clone()),
            ));
        }

        Ok(Self { statements })
    }

    pub fn run(&self) -> Result<(), RunTimeError> {
        let mut env = Environment::new();
        for stmt in &self.statements {
            stmt_eval::run(stmt, &mut env)?;
        }
        Ok(())
    }
}
