use std::str::Chars;

use crate::lexer::LexedToken;
use crate::syntax::OscSyntaxKind;

pub struct Cursor<'a> {
    source: Chars<'a>,
    token_offset: usize,
    source_offset: usize,
}

impl<'a> Cursor<'_> {
    pub fn new(src: &str) -> Cursor {
        Cursor {
            source: src.chars(),
            token_offset: 0,
            source_offset: 0,
        }
    }

    pub fn token_offset(&self) -> usize {
        self.token_offset
    }

    pub fn source_offset(&self) -> usize {
        self.source_offset
    }

    pub fn first(&self) -> Option<char> {
        self.source.clone().nth(self.token_offset + 0)
    }

    pub fn second(&self) -> Option<char> {
        self.source.clone().nth(self.token_offset + 1)
    }

    pub fn third(&self) -> Option<char> {
        self.source.clone().nth(self.token_offset + 2)
    }

    pub fn bump(&mut self) -> Option<char> {
        let offset = self.token_offset;
        self.token_offset += 1;
        self.source.clone().nth(offset)
    }

    pub fn eat(&mut self, c: char) -> bool {
        if self.first().is_some_and(|x| x == c) {
            self.bump();
            true
        } else {
            false
        }
    }

    pub fn token(&mut self, kind: OscSyntaxKind) -> LexedToken {
        let length = self.token_offset;
        self.token_offset = 0;
        self.source_offset += length;
        if length > 0 {
            self.source.nth(length - 1);
        }
        LexedToken { kind, length }
    }
}
