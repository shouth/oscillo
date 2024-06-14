use std::{
    collections::{HashSet, VecDeque},
    iter::Peekable,
};

use syntree::{pointer::PointerUsize, Builder, Checkpoint, Tree};

use crate::{
    lexer::{LexedToken, Lexer},
    syntax::OscDslSyntaxKind::{self, *},
};

pub struct LexicalAnalyzer<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    tokens: VecDeque<LexedToken>,
    offset: usize,
    indents: Vec<(&'a str, usize)>,
    enclosures: Vec<OscDslSyntaxKind>,
    is_new_line: bool,
    is_empty_line: bool,
}

impl<'a> LexicalAnalyzer<'a> {
    pub fn new(source: &'a str) -> LexicalAnalyzer<'a> {
        LexicalAnalyzer {
            source,
            lexer: Lexer::new(source),
            tokens: VecDeque::new(),
            offset: 0,
            indents: vec![("", 0)],
            enclosures: Vec::new(),
            is_new_line: true,
            is_empty_line: false,
        }
    }

    fn bump(&mut self) -> LexedToken {
        let token = self
            .tokens
            .pop_front()
            .unwrap_or_else(|| self.lexer.next_token());

        self.offset += token.length;
        token
    }

    fn alter(&mut self, kind: OscDslSyntaxKind) -> LexedToken {
        let token = self.bump();
        LexedToken { kind, ..token }
    }

    fn spawn(&self, kind: OscDslSyntaxKind) -> LexedToken {
        LexedToken { kind, length: 0 }
    }

    fn lookahead(&mut self, n: usize) -> &LexedToken {
        while self.tokens.len() <= n {
            let token = self.lexer.next_token();
            self.tokens.push_back(token);
        }
        &self.tokens[n]
    }

    pub fn next_token(&mut self) -> LexedToken {
        if self.is_new_line {
            self.is_new_line = false;

            let (indent, indent_width) = match self.lookahead(0).kind {
                TRIVIAL_NEWLINE => {
                    self.is_empty_line = true;
                    return self.next_token();
                }
                WHITESPACE => {
                    if self.lookahead(1).kind == TRIVIAL_NEWLINE {
                        self.is_empty_line = true;
                        return self.bump();
                    }

                    let whitespace = &self.source[self.offset..][..self.lookahead(0).length];
                    let indent_text_width = whitespace
                        .chars()
                        .take_while(|c| c == &' ' || c == &'\t')
                        .count();
                    let indent = &whitespace[..indent_text_width];
                    let indent_width = indent.chars().map(|c| if c == '\t' { 8 } else { 1 }).sum();
                    (indent, indent_width)
                }
                _ => ("", 0),
            };

            let (last_indent, last_indent_width) = self
                .indents
                .last()
                .expect("Indentation stack is empty")
                .to_owned();

            if indent_width > last_indent_width {
                self.indents.push((indent, indent_width));
                self.spawn(INDENT)
            } else if indent_width < last_indent_width {
                self.indents.pop();
                let (_, last_indent_width) = self
                    .indents
                    .last()
                    .expect("Indentation stack is empty")
                    .to_owned();

                if indent_width <= last_indent_width {
                    self.is_new_line = true;
                    self.spawn(DEDENT)
                } else {
                    self.spawn(ERROR)
                }
            } else if indent == last_indent {
                self.bump()
            } else {
                self.spawn(ERROR)
            }
        } else {
            match self.lookahead(0).kind.to_owned() {
                TRIVIAL_NEWLINE => {
                    self.is_new_line = true;
                    if !self.is_empty_line && self.enclosures.is_empty() {
                        self.alter(NEWLINE)
                    } else {
                        self.is_empty_line = false;
                        self.bump()
                    }
                }
                left @ (LEFT_BRACKET | LEFT_PAREN) => {
                    self.enclosures.push(left);
                    self.bump()
                }
                right @ (RIGHT_BRACKET | RIGHT_PAREN) => {
                    match self.enclosures.last().map(|left| (left, right)) {
                        Some((LEFT_BRACKET, RIGHT_BRACKET) | (LEFT_PAREN, RIGHT_PAREN)) => {
                            self.enclosures.pop();
                            self.bump()
                        }
                        _ => self.alter(ERROR),
                    }
                }
                _ => self.bump(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let source = r#"
scenario vehicle.two_phases:
    do serial (duration : [10s..30s]):
        phase1: actor.drive() with:
            speed(speed: 0kph, at: start)
            speed(speed: 10kph, at: end)

        phase2: actor.drive() with:
            speed(speed: [10kph..15kph])
"#;

        let mut lexer = LexicalAnalyzer::new(source);
        let mut offset = 0;
        loop {
            let token = lexer.next_token();
            let fragment = &source[offset..][..token.length];
            offset += token.length;
            println!("{:?} {:?}", token.kind, fragment);
            if token.kind == EOF {
                break;
            }
        }
    }
}
