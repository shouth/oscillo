use super::identifier_char::{is_id_char, is_id_start_char};
use std::str::Chars;

pub enum TokenKind {
    Identifier { closed: bool },
    String {},
    Integer {},
    Float,

    Dot,
    Comma,
    Colon,
    Assign,
    At,
    SingleArrow,
    VerticalLine,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    Question,
    DoubleArrow,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Indent {},
    Space {},
    NewLine {},

    Unknown,
}

pub struct Token {
    pub kind: TokenKind,
    pub length: usize,
}

struct Lexer<'a> {
    source: Chars<'a>,
    offset: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Lexer {
        Lexer {
            source: src.chars(),
            offset: 0,
        }
    }

    fn bump(&mut self) {
        self.offset += 1;
    }

    fn first(&self) -> Option<char> {
        self.source.clone().nth(self.offset)
    }

    fn second(&self) -> Option<char> {
        self.source.clone().nth(self.offset + 1)
    }

    fn token(&mut self, kind: TokenKind) -> Token {
        let offset = self.offset;
        self.offset = 0;
        self.source.nth(offset);
        Token {
            kind,
            length: offset,
        }
    }

    fn lex_identifier(&mut self) -> Option<Token> {
        match self.first() {
            Some(c) if is_id_start_char(c) => {
                self.bump();
                while let Some(c) = self.first() {
                    if !is_id_char(c) {
                        break;
                    }
                    self.bump();
                }
                Some(self.token(TokenKind::Identifier { closed: true }))
            }
            Some('|') => {
                self.bump();
                while let Some(c) = self.first() {
                    if c == '|' {
                        break;
                    }
                    self.bump();
                }

                if let Some('|') = self.first() {
                    self.bump();
                    Some(self.token(TokenKind::Identifier { closed: true }))
                } else {
                    Some(self.token(TokenKind::Identifier { closed: false }))
                }
            }
            Some(_) => {
                self.bump();
                Some(self.token(TokenKind::Unknown))
            }
            None => None,
        }
    }

    fn read_digits(&mut self, radix: u32) -> usize {
        let mut count = 0;
        while let Some(c) = self.first() {
            if c.is_digit(radix) {
                self.bump();
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    fn lex_float_with_exponent(&mut self) -> Option<Token> {
        match self.first() {
            Some('e' | 'E') => self.bump(),
            _ => return Some(self.token(TokenKind::Unknown)),
        };

        match self.first() {
            Some('+' | '-') => self.bump(),
            _ => (),
        };

        if self.read_digits(10) > 0 {
            Some(self.token(TokenKind::Float))
        } else {
            Some(self.token(TokenKind::Unknown))
        }
    }

    fn lex_float_with_fraction_and_exponent(&mut self) -> Option<Token> {
        match self.first() {
            Some('.') => self.bump(),
            _ => return Some(self.token(TokenKind::Unknown)),
        };

        if self.read_digits(10) == 0 {
            return Some(self.token(TokenKind::Unknown));
        }

        match self.first() {
            Some('e' | 'E') => self.lex_float_with_exponent(),
            _ => Some(self.token(TokenKind::Float)),
        }
    }

    fn lex_decimal_without_sign(&mut self, allow_integer: bool) -> Option<Token> {
        if self.read_digits(10) == 0 {
            return Some(self.token(TokenKind::Unknown));
        }

        match self.first() {
            Some('.') => self.lex_float_with_fraction_and_exponent(),
            Some('e' | 'E') => self.lex_float_with_exponent(),
            _ => {
                if allow_integer {
                    Some(self.token(TokenKind::Integer {}))
                } else {
                    Some(self.token(TokenKind::Unknown))
                }
            }
        }
    }

    fn lex_number(&mut self) -> Option<Token> {
        match self.first() {
            Some('+') => {
                self.bump();
                match self.first() {
                    Some('.') => self.lex_float_with_fraction_and_exponent(),
                    Some(c) if c.is_digit(10) => self.lex_decimal_without_sign(false),
                    _ => Some(self.token(TokenKind::Unknown)),
                }
            }
            Some('-') => {
                self.bump();
                match self.first() {
                    Some('.') => self.lex_float_with_fraction_and_exponent(),
                    Some(c) if c.is_digit(10) => self.lex_decimal_without_sign(true),
                    _ => Some(self.token(TokenKind::Unknown)),
                }
            }
            Some('.') => self.lex_float_with_fraction_and_exponent(),
            Some(c) if c.is_digit(10) => match self.second() {
                Some('x') => {
                    self.bump();
                    self.bump();
                    if self.read_digits(16) > 0 {
                        Some(self.token(TokenKind::Integer {}))
                    } else {
                        Some(self.token(TokenKind::Unknown))
                    }
                }
                _ => self.lex_decimal_without_sign(true),
            },
            Some(_) => Some(self.token(TokenKind::Unknown)),
            None => None,
        }
    }
}
