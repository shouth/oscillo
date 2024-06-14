use super::LexedToken;

pub trait LexedTokenSource {
    fn next_token(&mut self) -> LexedToken;
}

pub struct Lookahead<T: LexedTokenSource> {
    source: T,
    tokens: Vec<LexedToken>,
    offset: usize,
}

impl<T: LexedTokenSource> Lookahead<T> {
    pub fn new(source: T) -> Lookahead<T> {
        Lookahead {
            source,
            tokens: Vec::new(),
            offset: 0,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn bump(&mut self) -> LexedToken {
        let token = match self.tokens.pop() {
            Some(token) => token,
            None => self.source.next_token(),
        };
        self.offset += token.length;
        token
    }

    pub fn nth(&mut self, n: usize) -> &LexedToken {
        while self.tokens.len() <= n {
            self.tokens.push(self.source.next_token());
        }
        &self.tokens[n]
    }

    pub fn finish(self) -> T {
        self.source
    }
}
