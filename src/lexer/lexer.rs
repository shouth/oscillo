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

struct RawLexer<'a> {
    source: Chars<'a>,
    offset: usize,
}

impl<'a> RawLexer<'a> {
    fn new(src: &'a str) -> RawLexer {
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
                        _ => {
                            self.bump();
                        }
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
                        _ => {
                            self.bump();
                        }
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

    pub fn next(&mut self) -> Token {
        if let Some(c) = self.bump() {
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
                '"' | '\'' => {
                    self.eat_string(c);
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
    lexer: RawLexer<'a>,
    token: Token,
}

impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        let mut lexer = RawLexer::new(src);
        let token = lexer.next();
        Lexer { lexer, token }
    }

    fn lex(&mut self) -> Token {
        let mut token = self.lexer.next();
        std::mem::swap(&mut self.token, &mut token);
        token
    }

    fn peek(&self) -> &Token {
        &self.token
    }

    pub fn next(&mut self) -> Token {
        let token = self.lex();
        match token.kind {
            TokenKind::Plus => match self.peek().kind {
                TokenKind::Float => {
                    let next_token = self.lex();
                    Token {
                        kind: TokenKind::Float,
                        length: token.length + next_token.length,
                    }
                }
                _ => token,
            },
            TokenKind::Minus => match self.peek().kind {
                kind @ (TokenKind::Integer | TokenKind::Float) => {
                    let next_token = self.lex();
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
                let end = *offset + token.length;
                let lexeme = &source[*offset..end];
                *offset = end;
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
