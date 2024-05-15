use super::chars::{is_id_char, is_id_start_char};
use std::str::Chars;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum TokenKind {
    Identifier,
    String,
    Integer,
    Float,

    Dot,
    DotDot,
    Comma,
    Colon,
    ColonColon,
    Assign,
    At,
    SingleArrow,
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
    Exclamation,

    Indent,
    Dedent,
    Space,
    NewLine,
    Eos,

    Error,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub length: usize,
}

struct LowerLexer<'a> {
    source: Chars<'a>,
    offset: usize,
}

impl<'a> LowerLexer<'a> {
    fn new(src: &'a str) -> LowerLexer {
        LowerLexer {
            source: src.chars(),
            offset: 0,
        }
    }

    fn first(&self) -> Option<char> {
        self.source.clone().nth(self.offset + 0)
    }

    fn second(&self) -> Option<char> {
        self.source.clone().nth(self.offset + 1)
    }

    fn third(&self) -> Option<char> {
        self.source.clone().nth(self.offset + 2)
    }

    fn bump(&mut self) {
        self.offset += 1;
    }

    fn eat(&mut self, c: char) -> bool {
        if self.first().is_some_and(|x| x == c) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn eat_digits(&mut self, radix: u32) -> bool {
        if self.first().is_some_and(|c| c.is_digit(radix)) {
            while self.first().is_some_and(|c| c.is_digit(radix)) {
                self.bump();
            }
            true
        } else {
            false
        }
    }

    fn eat_exponent(&mut self) -> bool {
        if self.first().is_some_and(|c| c != 'e' && c != 'E') {
            false
        } else if self.second().is_some_and(|c| c == '+' || c == '-')
            && self.third().is_some_and(|c| c.is_digit(10))
        {
            // start with 'e+', 'e-', 'E+', 'E-'
            self.bump();
            self.bump();
            self.eat_digits(10);
            true
        } else if self.second().is_some_and(|c| c.is_digit(10)) {
            // start with 'e', 'E'
            self.bump();
            self.eat_digits(10);
            true
        } else {
            false
        }
    }

    fn eat_string(&mut self, delimiter: char) -> bool {
        if !self.eat(delimiter) {
            false
        } else if !self.eat(delimiter) {
            // nonempty short-string
            loop {
                if let Some(c) = self.first() {
                    match c {
                        '\\' => {
                            self.bump();
                            self.bump();
                        }
                        '\n' | '\r' => break,
                        _ if c == delimiter => {
                            self.bump();
                            break;
                        }
                        _ => self.bump(),
                    }
                } else {
                    break;
                }
            }
            true
        } else if !self.eat(delimiter) {
            // empty short-string
            true
        } else {
            // long-string
            loop {
                if let Some(c) = self.first() {
                    match c {
                        '\\' => {
                            self.bump();
                            self.bump();
                        }
                        _ if c == delimiter => {
                            self.bump();
                            break;
                        }
                        _ => self.bump(),
                    }
                } else {
                    break;
                }
            }
            true
        }
    }

    fn token(&mut self, kind: TokenKind) -> Token {
        let length = self.offset;
        self.offset = 0;
        if length > 0 {
            self.source.nth(length - 1);
        }
        Token { kind, length }
    }

    pub fn lex(&mut self) -> Token {
        if let Some(c) = self.first() {
            self.bump();
            match c {
                ' ' | '\t' => {
                    while self.eat(' ') || self.eat('\t') {}
                    self.token(TokenKind::Space)
                }
                '\n' => self.token(TokenKind::NewLine),
                '\r' => {
                    self.eat('\n');
                    self.token(TokenKind::NewLine)
                }
                ',' => self.token(TokenKind::Comma),
                '@' => self.token(TokenKind::At),
                '(' => self.token(TokenKind::LeftParenthesis),
                ')' => self.token(TokenKind::RightParenthesis),
                '[' => self.token(TokenKind::LeftBracket),
                ']' => self.token(TokenKind::RightBracket),
                '?' => self.token(TokenKind::Question),
                '+' => self.token(TokenKind::Plus),
                '*' => self.token(TokenKind::Star),
                '/' => self.token(TokenKind::Slash),
                '%' => self.token(TokenKind::Percent),
                ':' => {
                    if self.eat(':') {
                        self.token(TokenKind::ColonColon)
                    } else {
                        self.token(TokenKind::Colon)
                    }
                }
                '<' => {
                    if self.eat('=') {
                        self.token(TokenKind::LessThanEqual)
                    } else {
                        self.token(TokenKind::LessThan)
                    }
                }
                '>' => {
                    if self.eat('=') {
                        self.token(TokenKind::GreaterThanEqual)
                    } else {
                        self.token(TokenKind::GreaterThan)
                    }
                }
                '!' => {
                    if self.eat('=') {
                        self.token(TokenKind::NotEqual)
                    } else {
                        self.token(TokenKind::Exclamation)
                    }
                }
                '-' => {
                    if self.eat('>') {
                        self.token(TokenKind::SingleArrow)
                    } else {
                        self.token(TokenKind::Minus)
                    }
                }
                '=' => {
                    if self.eat('>') {
                        self.token(TokenKind::DoubleArrow)
                    } else if self.eat('=') {
                        self.token(TokenKind::Equal)
                    } else {
                        self.token(TokenKind::Assign)
                    }
                }
                '.' => {
                    if self.eat('.') {
                        self.token(TokenKind::DotDot)
                    } else if self.first().is_some_and(|c| c.is_digit(10)) {
                        self.eat_digits(10);
                        self.eat_exponent();
                        self.token(TokenKind::Float)
                    } else {
                        self.token(TokenKind::Dot)
                    }
                }
                '|' => {
                    while self.first().is_some_and(|c| c != '|') {
                        self.bump();
                    }
                    self.bump();
                    self.token(TokenKind::Identifier)
                }
                c @ ('"' | '\'') => {
                    self.eat_string(c);
                    self.token(TokenKind::String)
                }
                c @ '0'..='9' => {
                    if c == '0' && self.first().is_some_and(|c| c == 'x') {
                        if self.second().is_some_and(|c| c.is_digit(16)) {
                            // hexadecimal
                            self.bump();
                            self.eat_digits(16);
                            self.token(TokenKind::Integer)
                        } else {
                            // just '0'
                            self.token(TokenKind::Integer)
                        }
                    } else {
                        // decimal
                        self.eat_digits(10);
                        if let Some('.') = self.first() {
                            if self.second().is_some_and(|c| c.is_digit(10)) {
                                self.bump();
                                self.eat_digits(10);
                                self.eat_exponent();
                                self.token(TokenKind::Float)
                            } else {
                                self.token(TokenKind::Integer)
                            }
                        } else if self.eat_exponent() {
                            self.token(TokenKind::Float)
                        } else {
                            self.token(TokenKind::Integer)
                        }
                    }
                }
                c if is_id_start_char(c) => {
                    while self.first().is_some_and(is_id_char) {
                        self.bump();
                    }
                    self.token(TokenKind::Identifier)
                }
                _ => self.token(TokenKind::Error),
            }
        } else {
            self.token(TokenKind::Eos)
        }
    }
}

struct Lexer<'a> {
    lower: LowerLexer<'a>,
    token: Token,
}

impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        let mut lower = LowerLexer::new(src);
        let token = lower.lex();
        Lexer { lower, token }
    }

    fn lex(&mut self) -> Token {
        let mut token = self.lower.lex();
        std::mem::swap(&mut self.token, &mut token);
        token
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = self.lex();
        match (token.kind, self.token.kind) {
            (TokenKind::Plus, TokenKind::Float) => {
                let next_token = self.lex();
                Some(Token {
                    kind: TokenKind::Float,
                    length: token.length + next_token.length,
                })
            }
            (TokenKind::Minus, kind @ (TokenKind::Integer | TokenKind::Float)) => {
                let next_token = self.lex();
                Some(Token {
                    kind,
                    length: token.length + next_token.length,
                })
            }
            (TokenKind::Eos, _) => None,
            _ => Some(token),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn tokenize(source: &str) -> Vec<&str> {
        let lexer = Lexer::new(source);
        let tokens: Vec<Token> = lexer.collect();
        tokens
            .iter()
            .scan(0, |offset, token| {
                let end = *offset + token.length;
                let lexeme = &source[*offset..end];
                *offset = end;
                Some(lexeme)
            })
            .collect()
    }

    #[test]
    fn test_numeric() {
        assert_eq!(tokenize("0x"), vec!["0", "x"]);
        assert_eq!(tokenize("0x0"), vec!["0x0"]);

        assert_eq!(tokenize("123."), vec!["123", "."]);
        assert_eq!(tokenize("123.."), vec!["123", ".."]);
        assert_eq!(tokenize(".123"), vec![".123"]);
        assert_eq!(tokenize("..123"), vec!["..", "123"]);

        assert_eq!(tokenize("1.23"), vec!["1.23"]);
        assert_eq!(tokenize("+1.23"), vec!["+1.23"]);
        assert_eq!(tokenize("-1.23"), vec!["-1.23"]);

        assert_eq!(tokenize("1.23e456"), vec!["1.23e456"]);
        assert_eq!(tokenize("1.23e 456"), vec!["1.23", "e", " ", "456"]);
        assert_eq!(tokenize("1.23e+456"), vec!["1.23e+456"]);
        assert_eq!(tokenize("1.23ee+456"), vec!["1.23", "ee", "+", "456"]);
        assert_eq!(tokenize("1.23e +456"), vec!["1.23", "e", " ", "+", "456"]);
        assert_eq!(tokenize("1.23e-456"), vec!["1.23e-456"]);
        assert_eq!(tokenize("1.23ee-456"), vec!["1.23", "ee", "-456"]);
        assert_eq!(tokenize("1.23e -456"), vec!["1.23", "e", " ", "-456"]);
    }
}