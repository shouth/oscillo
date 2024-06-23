use crate::diagnostic::SyntaxDiagnostic;
use crate::syntax::OscSyntaxKind::{self, *};

use super::lookahead::{LookaheadSource, Lookahead};
use super::raw_lexer::RawLexer;
use super::LexedToken;

pub struct Lexer<'a> {
    inner: Lookahead<RawLexer<'a>>,
}

impl Lexer<'_> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            inner: Lookahead::new(RawLexer::new(source)),
        }
    }

    pub fn offset(&self) -> usize {
        self.inner.offset()
    }

    fn glue(&mut self, kind: OscSyntaxKind, prev: LexedToken) -> LexedToken {
        let next = self.inner.bump();
        LexedToken {
            kind,
            length: prev.length + next.length,
        }
    }

    pub fn next_token(&mut self) -> LexedToken {
        let token = self.inner.bump();

        // glue sign token and adjacent numeric token to meet max munch rule
        match token.kind {
            PLUS => match self.inner.nth(0).kind {
                kind @ FLOAT_LITERAL => self.glue(kind, token),
                _ => token,
            },
            MINUS => match self.inner.nth(0).kind {
                kind @ (INTEGER_LITERAL | FLOAT_LITERAL) => self.glue(kind, token),
                _ => token,
            },
            _ => token,
        }
    }

    pub fn finish(self) -> Vec<SyntaxDiagnostic> {
        self.inner.finish().finish()
    }
}

impl LookaheadSource for Lexer<'_> {
    fn next_token(&mut self) -> LexedToken {
        self.next_token()
    }
}
