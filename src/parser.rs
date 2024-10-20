use crate::{
    position::*,
    token::{Token, TokenKind},
};

static EOF_TOKEN: WithSpan<Token> = WithSpan::empty(Token::Eof);

pub struct Parser<'a> {
    current: usize,
    tokens: &'a Vec<WithSpan<Token>>,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<WithSpan<Token>>) -> Self {
        Parser {
            current: 0,
            tokens: tokens,
            diagnostics: Vec::new(),
        }
    }

    pub fn had_error(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn error(&mut self, message: &str, span: Span) {
        self.diagnostics.push(Diagnostic {
            message: message.to_string(),
            span,
        });
    }

    pub fn peek(&self) -> TokenKind {
        return self.peek_token().value.kind();
    }

    pub fn peek_token(&self) -> &'a WithSpan<Token> {
        self.tokens.get(self.current).unwrap_or(&EOF_TOKEN)
    }

    pub fn previous(&self) -> &'a WithSpan<Token> {
        return self.tokens.get(self.current - 1).unwrap_or(&EOF_TOKEN);
    }

    pub fn is_at_end(&self) -> bool {
        return self.peek() == TokenKind::Eof;
    }

    pub fn check(&self, token: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            token == self.peek().into()
        }
    }

    pub fn advance(&mut self) -> &'a WithSpan<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    pub fn one_of<T: IntoIterator<Item = TokenKind>>(&mut self, tokens: T) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    pub fn is(&mut self, token: TokenKind) -> bool {
        if self.check(token.into()) {
            self.advance();
            return true;
        }
        return false;
    }

    pub fn expect(&mut self, expected: TokenKind) -> Option<&'a WithSpan<Token>> {
        let token = self.advance();
        if expected == token.value.kind() {
            return Some(token);
        } else {
            self.error(
                &format!("Expected {} got {}", expected, token.value),
                token.span,
            );
            return None;
        }
    }

    pub fn optional(&mut self, token: TokenKind) -> bool {
        if self.check(token) {
            self.advance();
            return true;
        }
        return false;
    }
}
