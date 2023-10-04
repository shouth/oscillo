use std::str::Chars;
use super::identifier_char::{is_id_char, is_id_start_char};

pub enum TokenKind {
    Identifier { },
    String { },
    Integer { },
    Float { },

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
}

pub struct Token {
    pub kind: TokenKind,
    pub length: usize,
}

impl Token {
    fn new(kind: TokenKind, length: usize) -> Token {
        Token { kind, length }
    }
}

pub const EOF: char = '\u{0000}';

struct Lexer<'a> {
    source: Chars<'a>,
    offset: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Lexer {
        Lexer { source: src.chars(), offset: 0 }
    }

    fn bump(&mut self) {
        self.offset += 1;
    }

    fn first(&self) -> char {
        self.source.clone().nth(self.offset).unwrap_or(EOF)
    }

    fn second(&self) -> char {
        self.source.clone().nth(self.offset + 1).unwrap_or(EOF)
    }

    fn token(&mut self, kind: TokenKind) -> Token {
        let offset = self.offset;
        self.offset = 0;
        self.source.nth(offset);
        Token::new(kind, offset)
    }

    fn lex_identifier(&mut self) -> Option<Token> {
        match self.first() {
            c if is_id_start_char(c) => {
                self.bump();
                while is_id_char(self.first()) {
                    self.bump();
                }
                Some(self.token(TokenKind::Identifier {  }))
            }
            '|' => {
                self.bump();
                while self.first() != '|' {
                    self.bump();
                }

                if self.first() == '|' {
                    self.bump();
                    Some(self.token(TokenKind::Identifier {  }))
                } else {
                    None
                }
            }
            _ => None
        }
    }

    fn lex_integer(&mut self) -> Option<Token> {
        match self.first() {
            '0' if self.second() == 'x' => {
                self.bump();
                self.bump();
                while self.first().is_digit(16) {
                    self.bump();
                }
                Some(self.token(TokenKind::Integer {  }))
            }
            '-' => {
                self.bump();
                while self.first().is_digit(10) {
                    self.bump();
                }
                Some(self.token(TokenKind::Integer {  }))
            }
            c if c.is_digit(10) => {
                self.bump();
                while self.first().is_digit(10) {
                    self.bump();
                }
                Some(self.token(TokenKind::Integer {  }))
            },
            _ => None
        }
    }

    fn lex_float(&mut self) -> Option<Token> {
        if self.first() == '+' {
            self.bump();
        } else if self.first() == '-' {
            self.bump();
        } else {
            
        }

        while self.first().is_digit(10) {
            self.bump();
        }

        if self.first() != '.' {
            return None;
        }
        self.bump();

        if self.first().is_digit(10) {
            return None;
        }
        self.bump();

        while self.first().is_digit(10) {
            self.bump();
        }

        if self.first() == 'e' || self.second() == 'E' {
            self.bump();
            if self.first() == '+' || self.first() == '-' {
                if !self.first().is_digit(10) {
                    return None;
                }
                self.bump();
                while self.first().is_digit(10) {
                    self.bump();
                }
            }
        }

        Some(self.token(TokenKind::Float {  }))
    }
}
