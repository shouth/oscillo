use std::{collections::VecDeque, str::Chars};

use crate::chars::{is_id_char, is_id_start_char};
use crate::diagnostic::Diagnostic;
use crate::syntax::OscDslSyntaxKind::{self, *};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct LexedToken {
    pub kind: OscDslSyntaxKind,
    pub length: usize,
}

struct Cursor<'a> {
    source: Chars<'a>,
    token_offset: usize,
    source_offset: usize,
}

impl<'a> Cursor<'_> {
    fn new(src: &str) -> Cursor {
        Cursor {
            source: src.chars(),
            token_offset: 0,
            source_offset: 0,
        }
    }

    fn token_offset(&self) -> usize {
        self.token_offset
    }

    fn source_offset(&self) -> usize {
        self.source_offset
    }

    fn first(&self) -> Option<char> {
        self.source.clone().nth(self.token_offset + 0)
    }

    fn second(&self) -> Option<char> {
        self.source.clone().nth(self.token_offset + 1)
    }

    fn third(&self) -> Option<char> {
        self.source.clone().nth(self.token_offset + 2)
    }

    fn bump(&mut self) -> Option<char> {
        let offset = self.token_offset;
        self.token_offset += 1;
        self.source.clone().nth(offset)
    }

    fn eat(&mut self, c: char) -> bool {
        if self.first().is_some_and(|x| x == c) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn token(&mut self, kind: OscDslSyntaxKind) -> LexedToken {
        let length = self.token_offset;
        self.token_offset = 0;
        self.source_offset += length;
        if length > 0 {
            self.source.nth(length - 1);
        }
        LexedToken { kind, length }
    }
}

pub struct RawLexer<'a> {
    cursor: Cursor<'a>,
    diagnostics: Vec<Diagnostic>,
}

impl RawLexer<'_> {
    pub fn new(source: &str) -> RawLexer {
        RawLexer {
            cursor: Cursor::new(source),
            diagnostics: Vec::new(),
        }
    }

    pub fn offset(&self) -> usize {
        self.cursor.source_offset()
    }

    fn eat_digits(&mut self, radix: u32) -> bool {
        if self.cursor.first().is_some_and(|c| c.is_digit(radix)) {
            while self.cursor.first().is_some_and(|c| c.is_digit(radix)) {
                self.cursor.bump();
            }
            true
        } else {
            false
        }
    }

    fn eat_exponent(&mut self) -> bool {
        if self.cursor.first().is_some_and(|c| c != 'e' && c != 'E') {
            false
        } else if self.cursor.second().is_some_and(|c| c == '+' || c == '-')
            && self.cursor.third().is_some_and(|c| c.is_digit(10))
        {
            // start with 'e+', 'e-', 'E+', 'E-'
            self.cursor.bump();
            self.cursor.bump();
            self.eat_digits(10);
            true
        } else if self.cursor.second().is_some_and(|c| c.is_digit(10)) {
            // start with 'e', 'E'
            self.cursor.bump();
            self.eat_digits(10);
            true
        } else {
            false
        }
    }

    pub fn next_token(&mut self) -> LexedToken {
        if let Some(c) = self.cursor.first() {
            self.cursor.bump();
            match c {
                '\t' | '\u{000C}' | ' ' | '\\' => {
                    if c == '\\' {
                        if self.cursor.eat('\r') {
                            self.cursor.eat('\n');
                        } else if self.cursor.eat('\n') {
                            // do nothing
                        } else {
                            let start = self.cursor.source_offset();
                            let end = start + self.cursor.token_offset();
                            self.diagnostics.push(Diagnostic::new(
                                start..end,
                                "stray `\\` in program",
                            ));
                            return self.cursor.token(ERROR);
                        }
                    }

                    while let Some(c) = self.cursor.first() {
                        match c {
                            '\t' | '\u{000C}' | ' ' => {
                                self.cursor.bump();
                            }
                            '\\' => {
                                if self.cursor.second().is_some_and(|c| c == '\r') {
                                    self.cursor.bump();
                                    self.cursor.bump();
                                    self.cursor.eat('\n');
                                } else if self.cursor.second().is_some_and(|c| c == '\n') {
                                    self.cursor.bump();
                                    self.cursor.bump();
                                } else {
                                    break;
                                }
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    self.cursor.token(WHITESPACE)
                }
                '\n' => self.cursor.token(TRIVIAL_NEWLINE),
                '\r' => {
                    self.cursor.eat('\n');
                    self.cursor.token(TRIVIAL_NEWLINE)
                }
                '#' => {
                    while self.cursor.first().is_some_and(|c| c != '\n' && c != '\r') {
                        self.cursor.bump();
                    }
                    self.cursor.token(COMMENT)
                }
                '|' => {
                    while self.cursor.first().is_some_and(|c| c != '|') {
                        self.cursor.bump();
                    }
                    self.cursor.bump();
                    self.cursor.token(IDENTIFIER)
                }
                c if is_id_start_char(c) => {
                    while self.cursor.first().is_some_and(is_id_char) {
                        self.cursor.bump();
                    }
                    self.cursor.token(IDENTIFIER)
                }
                '"' | '\'' => {
                    if !self.cursor.eat(c) {
                        // non-empty string
                        while let Some(e) = self.cursor.first() {
                            match e {
                                '\\' => {
                                    self.cursor.bump();
                                    self.cursor.bump();
                                }
                                '\n' | '\r' => break,
                                _ if e == c => {
                                    self.cursor.bump();
                                    break;
                                }
                                _ => {
                                    self.cursor.bump();
                                }
                            }
                        }
                    } else if !self.cursor.eat(c) {
                        // empty string
                    } else {
                        // long string
                        while let Some(e) = self.cursor.first() {
                            match e {
                                '\\' => {
                                    self.cursor.bump();
                                    self.cursor.bump();
                                }
                                _ if e == c => {
                                    self.cursor.bump();
                                    if self.cursor.eat(c) && self.cursor.eat(c) {
                                        break;
                                    }
                                }
                                _ => {
                                    self.cursor.bump();
                                }
                            }
                        }
                    }
                    self.cursor.token(STRING_LITERAL)
                }
                '0'..='9' => {
                    if c == '0' && self.cursor.first().is_some_and(|c| c == 'x') {
                        if self.cursor.second().is_some_and(|c| c.is_digit(16)) {
                            // hexadecimal
                            self.cursor.bump();
                            self.eat_digits(16);
                            self.cursor.token(INTEGER_LITERAL)
                        } else {
                            // just '0'
                            self.cursor.token(INTEGER_LITERAL)
                        }
                    } else {
                        // decimal
                        self.eat_digits(10);
                        if let Some('.') = self.cursor.first() {
                            if self.cursor.second().is_some_and(|c| c.is_digit(10)) {
                                self.cursor.bump();
                                self.eat_digits(10);
                                self.eat_exponent();
                                self.cursor.token(FLOAT_LITERAL)
                            } else {
                                self.cursor.token(INTEGER_LITERAL)
                            }
                        } else if self.eat_exponent() {
                            self.cursor.token(FLOAT_LITERAL)
                        } else {
                            self.cursor.token(INTEGER_LITERAL)
                        }
                    }
                }
                '.' => {
                    if self.cursor.eat('.') {
                        self.cursor.token(DOT_DOT)
                    } else if self.cursor.first().is_some_and(|c| c.is_digit(10)) {
                        self.eat_digits(10);
                        self.eat_exponent();
                        self.cursor.token(FLOAT_LITERAL)
                    } else {
                        self.cursor.token(DOT)
                    }
                }
                ',' => self.cursor.token(COMMA),
                '@' => self.cursor.token(AT),
                '(' => self.cursor.token(LEFT_PAREN),
                ')' => self.cursor.token(RIGHT_PAREN),
                '[' => self.cursor.token(LEFT_BRACKET),
                ']' => self.cursor.token(RIGHT_BRACKET),
                '?' => self.cursor.token(QUESTION),
                '+' => self.cursor.token(PLUS),
                '*' => self.cursor.token(STAR),
                '/' => self.cursor.token(SLASH),
                '%' => self.cursor.token(PERCENT),
                ':' => {
                    if self.cursor.eat(':') {
                        self.cursor.token(COLON_COLON)
                    } else {
                        self.cursor.token(COLON)
                    }
                }
                '<' => {
                    if self.cursor.eat('=') {
                        self.cursor.token(LESS_EQUAL)
                    } else {
                        self.cursor.token(LESS)
                    }
                }
                '>' => {
                    if self.cursor.eat('=') {
                        self.cursor.token(GREATER_EQUAL)
                    } else {
                        self.cursor.token(GREATER)
                    }
                }
                '!' => {
                    if self.cursor.eat('=') {
                        self.cursor.token(NOT_EQUAL)
                    } else {
                        self.cursor.token(EXCLAMATION)
                    }
                }
                '-' => {
                    if self.cursor.eat('>') {
                        self.cursor.token(ARROW)
                    } else {
                        self.cursor.token(MINUS)
                    }
                }
                '=' => {
                    if self.cursor.eat('>') {
                        self.cursor.token(FAT_ARROW)
                    } else if self.cursor.eat('=') {
                        self.cursor.token(EQUAL)
                    } else {
                        self.cursor.token(ASSIGN)
                    }
                }
                _ => {
                    let start = self.cursor.source_offset();
                    let end = start + self.cursor.token_offset();
                    self.diagnostics.push(Diagnostic::new(
                        start..end,
                        format!("unexpected character `{}`", c),
                    ));
                    self.cursor.token(ERROR)
                }
            }
        } else {
            self.cursor.token(EOF)
        }
    }

    pub fn finish(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

pub struct Lexer<'a> {
    inner: RawLexer<'a>,
    tokens: VecDeque<LexedToken>,
}

impl Lexer<'_> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            inner: RawLexer::new(source),
            tokens: VecDeque::new(),
        }
    }

    pub fn offset(&self) -> usize {
        self.inner.offset()
    }

    fn bump(&mut self) -> LexedToken {
        match self.tokens.pop_front() {
            Some(token) => token,
            None => self.inner.next_token(),
        }
    }

    fn lookahead(&mut self, n: usize) -> &LexedToken {
        while self.tokens.len() <= n {
            self.tokens.push_back(self.inner.next_token());
        }
        &self.tokens[n]
    }

    fn glue(&mut self, kind: OscDslSyntaxKind, prev: LexedToken) -> LexedToken {
        let next = self.bump();
        LexedToken {
            kind,
            length: prev.length + next.length,
        }
    }

    pub fn next_token(&mut self) -> LexedToken {
        let token = self.bump();

        // glue sign token and adjacent numeric token to meet max munch rule
        match token.kind {
            PLUS => match self.lookahead(0).kind {
                kind @ FLOAT_LITERAL => self.glue(kind, token),
                _ => token,
            },
            MINUS => match self.lookahead(0).kind {
                kind @ (INTEGER_LITERAL | FLOAT_LITERAL) => self.glue(kind, token),
                _ => token,
            },
            _ => token,
        }
    }

    pub fn finish(self) -> Vec<Diagnostic> {
        self.inner.finish()
    }
}

pub struct LexicalAnalyzer<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    tokens: VecDeque<LexedToken>,
    indents: Vec<(&'a str, usize)>,
    enclosures: Vec<OscDslSyntaxKind>,
    is_new_line: bool,
    is_empty_line: bool,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> LexicalAnalyzer<'a> {
    pub fn new(source: &'a str) -> LexicalAnalyzer<'a> {
        LexicalAnalyzer {
            source,
            lexer: Lexer::new(source),
            tokens: VecDeque::new(),
            indents: vec![("", 0)],
            enclosures: Vec::new(),
            is_new_line: true,
            is_empty_line: false,
            diagnostics: Vec::new(),
        }
    }

    pub fn offset(&self) -> usize {
        self.lexer.offset()
    }

    fn bump(&mut self) -> LexedToken {
        match self.tokens.pop_front() {
            Some(token) => token,
            None => self.lexer.next_token(),
        }
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

                    let whitespace = &self.source[self.offset()..][..self.lookahead(0).length];
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
                    let start = self.offset();
                    let end = start + self.lookahead(0).length;
                    self.diagnostics.push(Diagnostic::new(
                        start..end,
                        format!("indentation is not aligned with the previous line"),
                    ));
                    self.spawn(ERROR)
                }
            } else if indent == last_indent {
                self.bump()
            } else {
                let start = self.offset();
                let end = start + self.lookahead(0).length;
                self.diagnostics.push(Diagnostic::new(
                    start..end,
                    format!("indentation is not aligned with the previous line"),
                ));
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
                        }
                        _ => {
                            // enclosures are mismatched.
                            // clear enclosures to make suceeding TRIVIAL_NEWLINE to be treated as NEWLINE.
                            // making error from mismatched enclosures is parser's responsibility.
                            self.enclosures.clear();
                        }
                    }
                    self.bump()
                }
                _ => self.bump(),
            }
        }
    }

    pub fn finish(self) -> Vec<Diagnostic> {
        let mut diagnostics = self.diagnostics;
        diagnostics.extend(self.lexer.finish());
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(source: &str) -> Vec<&str> {
        let mut result = Vec::new();
        let mut lexer = Lexer::new(source);
        let mut offset = 0;
        loop {
            let token = lexer.next_token();
            if token.kind == super::EOF {
                return result;
            }
            result.push(&source[offset..][..token.length]);
            offset += token.length;
        }
    }

    #[test]
    fn test_numeric() {
        assert_eq!(tokenize("0x"), ["0", "x"]);
        assert_eq!(tokenize("0x0"), ["0x0"]);

        assert_eq!(tokenize("123."), ["123", "."]);
        assert_eq!(tokenize("123.."), ["123", ".."]);
        assert_eq!(tokenize(".123"), [".123"]);
        assert_eq!(tokenize("..123"), ["..", "123"]);

        assert_eq!(tokenize("1.23"), ["1.23"]);
        assert_eq!(tokenize("+1.23"), ["+1.23"]);
        assert_eq!(tokenize("-1.23"), ["-1.23"]);

        assert_eq!(tokenize("1.23e456"), ["1.23e456"]);
        assert_eq!(tokenize("1.23e 456"), ["1.23", "e", " ", "456"]);
        assert_eq!(tokenize("1.23e+456"), ["1.23e+456"]);
        assert_eq!(tokenize("1.23ee+456"), ["1.23", "ee", "+", "456"]);
        assert_eq!(tokenize("1.23e +456"), ["1.23", "e", " ", "+", "456"]);
        assert_eq!(tokenize("1.23e-456"), ["1.23e-456"]);
        assert_eq!(tokenize("1.23ee-456"), ["1.23", "ee", "-456"]);
        assert_eq!(tokenize("1.23e -456"), ["1.23", "e", " ", "-456"]);
    }

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
