use crate::diagnostic::SyntaxDiagnostic;
use crate::lexer::raw_lexer::RawLexer;
use crate::lexer::LexedToken;
use crate::syntax::OscSyntaxKind::{self, *};

pub struct Lexer<'a> {
    lexer: RawLexer<'a>,
    next: LexedToken,
}

impl Lexer<'_> {
    pub fn new(source: &str) -> Lexer {
        let mut lexer = RawLexer::new(source);
        let next = lexer.next_token();
        Lexer { lexer, next }
    }

    pub fn offset(&self) -> usize {
        self.lexer.offset() - self.next.length
    }

    fn bump(&mut self) -> LexedToken {
        let current = self.next;
        self.next = self.lexer.next_token();
        current
    }

    fn glue(&mut self, kind: OscSyntaxKind, prev: LexedToken) -> LexedToken {
        let current = self.bump();
        LexedToken {
            kind,
            length: prev.length + current.length,
        }
    }

    pub fn next_token(&mut self) -> LexedToken {
        let current = self.bump();

        // glue sign token and adjacent numeric token to meet max munch rule
        match current.kind {
            PLUS => match self.next.kind {
                kind @ FLOAT_LITERAL => self.glue(kind, current),
                _ => current,
            },
            MINUS => match self.next.kind {
                kind @ (INTEGER_LITERAL | FLOAT_LITERAL) => self.glue(kind, current),
                _ => current,
            },
            _ => current,
        }
    }

    pub fn finish(self) -> Vec<SyntaxDiagnostic> {
        self.lexer.finish()
    }
}
