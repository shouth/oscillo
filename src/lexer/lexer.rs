use super::chars::{is_id_char, is_id_start_char};
use std::str::Chars;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum TokenKind {
    Space,
    FormFeed,
    PhysicalNewLine,
    LogicalNewLine,
    Comment,
    Indent,
    Dedent,

    Identifier,
    String,
    Integer,
    Float,

    Backslash,
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

    Eos,

    Error,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub length: usize,
}

struct RawLexer<'a> {
    source: Chars<'a>,
    offset: usize,
}

impl<'a> RawLexer<'_> {
    fn new(src: &str) -> RawLexer {
        RawLexer {
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

    fn bump(&mut self) -> Option<char> {
        let offset = self.offset;
        self.offset += 1;
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

    fn token(&mut self, kind: TokenKind) -> Token {
        let length = self.offset;
        self.offset = 0;
        if length > 0 {
            self.source.nth(length - 1);
        }
        Token { kind, length }
    }

    pub fn next(&mut self) -> Token {
        if let Some(c) = self.bump() {
            match c {
                ' ' | '\t' => {
                    while self.eat(' ') || self.eat('\t') {}
                    self.token(TokenKind::Space)
                }
                '\u{000C}' => self.token(TokenKind::FormFeed),
                '\n' => self.token(TokenKind::PhysicalNewLine),
                '\r' => {
                    self.eat('\n');
                    self.token(TokenKind::PhysicalNewLine)
                }
                '\\' => {
                    if let Some('\n' | '\r') = self.first() {
                        self.token(TokenKind::Backslash)
                    } else {
                        self.token(TokenKind::Error)
                    }
                }
                '#' => {
                    while self.first().is_some_and(|c| c != '\n' && c != '\r') {
                        self.bump();
                    }
                    self.token(TokenKind::Comment)
                }
                '|' => {
                    while self.first().is_some_and(|c| c != '|') {
                        self.bump();
                    }
                    self.bump();
                    self.token(TokenKind::Identifier)
                }
                c if is_id_start_char(c) => {
                    while self.first().is_some_and(is_id_char) {
                        self.bump();
                    }
                    self.token(TokenKind::Identifier)
                }
                '"' | '\'' => {
                    if !self.eat(c) {
                        // non-empty string
                        while let Some(e) = self.first() {
                            match e {
                                '\\' => {
                                    self.bump();
                                    self.bump();
                                }
                                '\n' | '\r' => break,
                                _ if e == c => {
                                    self.bump();
                                    break;
                                }
                                _ => {
                                    self.bump();
                                }
                            }
                        }
                    } else if !self.eat(c) {
                        // empty string
                    } else {
                        // long string
                        while let Some(e) = self.first() {
                            match e {
                                '\\' => {
                                    self.bump();
                                    self.bump();
                                }
                                _ if e == c => {
                                    self.bump();
                                    if self.eat(c) && self.eat(c) {
                                        break;
                                    }
                                }
                                _ => {
                                    self.bump();
                                }
                            }
                        }
                    }
                    self.token(TokenKind::String)
                }
                '0'..='9' => {
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
                _ => self.token(TokenKind::Error),
            }
        } else {
            self.token(TokenKind::Eos)
        }
    }
}

struct Lexer<'a> {
    lexer: RawLexer<'a>,
    token: Token,
}

impl Lexer<'_> {
    pub fn new(source: &str) -> Lexer {
        let mut lexer = RawLexer::new(source);
        let token = lexer.next();
        Lexer { lexer, token }
    }

    fn bump(&mut self) -> Token {
        let mut token = self.lexer.next();
        std::mem::swap(&mut self.token, &mut token);
        token
    }

    fn peek(&self) -> &Token {
        &self.token
    }

    pub fn next(&mut self) -> Token {
        let token = self.bump();

        // glue sign token and adjacent numeric token to meet max munch rule
        match token.kind {
            TokenKind::Plus => match self.peek().kind {
                TokenKind::Float => {
                    let next_token = self.bump();
                    Token {
                        kind: TokenKind::Float,
                        length: token.length + next_token.length,
                    }
                }
                _ => token,
            },
            TokenKind::Minus => match self.peek().kind {
                kind @ (TokenKind::Integer | TokenKind::Float) => {
                    let next_token = self.bump();
                    Token {
                        kind,
                        length: token.length + next_token.length,
                    }
                }
                _ => token,
            },
            _ => token,
        }
    }
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lexer = Lexer::new(source);
    std::iter::from_fn(move || {
        let token = lexer.next();
        if token.kind == TokenKind::Eos {
            None
        } else {
            Some(token)
        }
    })
}

#[cfg(test)]
mod tests {
    fn tokenize(source: &str) -> Vec<&str> {
        super::tokenize(source)
            .scan(0, |offset, token| {
                let lexeme = &source[*offset..][..token.length];
                *offset += token.length;
                Some(lexeme)
            })
            .collect()
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
}
