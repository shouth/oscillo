use std::collections::VecDeque;

use super::LexedToken;

pub trait LookaheadSource {
    fn next_token(&mut self) -> LexedToken;
}

pub struct Lookahead<T: LookaheadSource> {
    source: T,
    tokens: VecDeque<LexedToken>,
    offset: usize,
}

impl<T: LookaheadSource> Lookahead<T> {
    pub fn new(source: T) -> Lookahead<T> {
        Lookahead {
            source,
            tokens: VecDeque::new(),
            offset: 0,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn bump(&mut self) -> LexedToken {
        let token = match self.tokens.pop_front() {
            Some(token) => token,
            None => self.source.next_token(),
        };
        self.offset += token.length;
        token
    }

    pub fn nth(&mut self, n: usize) -> &LexedToken {
        while self.tokens.len() <= n {
            self.tokens.push_back(self.source.next_token());
        }
        &self.tokens[n]
    }

    pub fn finish(self) -> T {
        self.source
    }
}
