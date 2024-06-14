use super::lookahead::LookaheadSource;
use super::{cursor::Cursor, LexedToken};

use crate::chars::{is_id_char, is_id_start_char};
use crate::diagnostic::Diagnostic;
use crate::syntax::OscDslSyntaxKind::*;

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
                            self.diagnostics
                                .push(Diagnostic::new(start..end, "stray `\\` in program"));
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

impl LookaheadSource for RawLexer<'_> {
    fn next_token(&mut self) -> LexedToken {
        self.next_token()
    }
}
