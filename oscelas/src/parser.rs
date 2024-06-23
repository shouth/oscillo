mod common;
mod decl;
mod expr;
mod member;
mod osc_file;

use syntree::pointer::PointerUsize;
use syntree::{Builder as TreeBuilder, Checkpoint as TreeCheckpoint, Tree};

use crate::diagnostic::SyntaxDiagnostic;
use crate::lexer::{LexicalAnalyzer, Lookahead};
use crate::syntax::OscSyntaxKind::{self, *};
use crate::syntax::OscSyntaxKindSet;

#[derive(Debug, Clone)]
pub struct Checkpoint(TreeCheckpoint<PointerUsize>);

pub struct Parser<'a> {
    source: &'a str,
    lexer: Lookahead<LexicalAnalyzer<'a>>,
    builder: TreeBuilder<OscSyntaxKind, u32, usize>,
    diagnostic: Vec<SyntaxDiagnostic>,
    expected: OscSyntaxKindSet,
    leading_trivia: bool,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        let mut parser = Parser {
            source,
            lexer: Lookahead::new(LexicalAnalyzer::new(source)),
            builder: TreeBuilder::new(),
            diagnostic: Vec::new(),
            expected: OscSyntaxKindSet::new(),
            leading_trivia: false,
        };
        parser.skip_trivia();
        parser
    }

    fn skip_trivia(&mut self) {
        while let kind @ (WHITESPACE | COMMENT | TRIVIAL_NEWLINE) = self.lexer.nth(0).kind {
            self.bump(kind);
            self.leading_trivia = true;
        }
    }

    pub fn has_leading_trivia(&self) -> bool {
        self.leading_trivia
    }

    pub fn diagnostic(&mut self, diagnostic: SyntaxDiagnostic) {
        self.diagnostic.push(diagnostic);
    }

    pub fn bump(&mut self, kind: OscSyntaxKind) {
        let token = self.lexer.bump();
        self.builder.token(kind, token.length).unwrap();
        self.expected.clear();
        self.leading_trivia = false;
        self.skip_trivia();
    }

    fn do_check(&mut self, kinds: OscSyntaxKindSet) -> Option<OscSyntaxKind> {
        let next = self.lexer.nth(0).kind;

        let keyword = match next {
            IDENTIFIER => {
                let begin = self.lexer.offset();
                let length = self.lexer.nth(0).length;
                let lexeme = &self.source[begin..][..length];
                kinds.iter().find(|kind| kind.static_token() == Some(lexeme))
            }
            _ => None,
        };

        match keyword {
            Some(_) => keyword,
            None => kinds.contains(next).then(|| next),
        }
    }

    pub fn check(&mut self, kinds: impl Into<OscSyntaxKindSet>) -> bool {
        self.do_check(kinds.into()).is_some()
    }

    pub fn eat(&mut self, kinds: impl Into<OscSyntaxKindSet>) -> bool {
        let result = self.do_check(kinds.into());
        if let Some(kind) = result {
            self.bump(kind);
        }
        result.is_some()
    }

    pub fn expect(&mut self, kinds: impl Into<OscSyntaxKindSet>) -> bool {
        let result = self.eat(kinds);
        if !result {
            self.unexpected();
        }
        result
    }

    pub fn unexpected(&mut self) {
        let start = self.lexer.offset();
        let end = start + self.lexer.nth(0).length;
        let expected = self.expected;
        let actual = self.lexer.nth(0).kind;

        self.diagnostic(SyntaxDiagnostic::UnexpectedToken {
            range: start..end,
            expected,
            actual,
        });
    }

    pub fn recover(&mut self, kinds: impl Into<OscSyntaxKindSet>) {
        let kinds = kinds.into();
        while !self.check(kinds) {
            self.lexer.bump();
        }
    }

    pub fn open(&mut self) -> Checkpoint {
        Checkpoint(self.builder.checkpoint().unwrap())
    }

    pub fn close(&mut self, checkpoint: Checkpoint, kind: OscSyntaxKind) {
        self.builder.close_at(&checkpoint.0, kind).unwrap();
    }

    pub fn finish(mut self) -> (Vec<SyntaxDiagnostic>, Tree<OscSyntaxKind, u32, usize>) {
        loop {
            self.skip_trivia();
            let token = self.lexer.bump();
            if token.kind == EOF {
                self.builder.token(EOF, 0).unwrap();
                break;
            }
            self.builder.token(ERROR, token.length).unwrap();
        }

        let mut diagnostics = Vec::new();
        diagnostics.extend(self.lexer.finish().finish());
        diagnostics.extend(self.diagnostic);

        (diagnostics, self.builder.build().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use expr::parse_expr;
    use osc_file::parse_osc_file;

    use super::*;

    #[test]
    fn test_parse_expr() -> Result<(), Box<dyn std::error::Error>> {
        let source = "not not (-x in [1, 2, 3]) and y > 3.14 or 42 % 3 == 1 => object.method(a[i], x: 2 * pi)";
        let expected = syntree::tree! {
            LOGICAL_EXP => {
                LOGICAL_EXP => {
                    LOGICAL_EXP => {
                        UNARY_EXP => {
                            (NOT_KW, 3),
                            (WHITESPACE, 1),
                            UNARY_EXP => {
                                (NOT_KW, 3),
                                (WHITESPACE, 1),
                                PARENTHESIZED_EXP => {
                                    (LEFT_PAREN, 1),
                                    BINARY_EXP => {
                                        UNARY_EXP => {
                                            (MINUS, 1),
                                            (IDENTIFIER, 1),
                                            (WHITESPACE, 1),
                                        },
                                        (IN_KW, 2),
                                        (WHITESPACE, 1),
                                        LIST_CONSTRUCTOR => {
                                            (LEFT_BRACKET, 1),
                                            EXPRESSION_LIST => {
                                                EXPRESSION_LIST_ELEMENT => {
                                                    (INTEGER_LITERAL, 1),
                                                    (COMMA, 1),
                                                    (WHITESPACE, 1),
                                                },
                                                EXPRESSION_LIST_ELEMENT => {
                                                    (INTEGER_LITERAL, 1),
                                                    (COMMA, 1),
                                                    (WHITESPACE, 1),
                                                },
                                                EXPRESSION_LIST_ELEMENT => {
                                                    (INTEGER_LITERAL, 1),
                                                },
                                            },
                                            (RIGHT_BRACKET, 1),
                                        },
                                    },
                                    (RIGHT_PAREN, 1),
                                    (WHITESPACE, 1),
                                },
                            },
                        },
                        (AND_KW, 3),
                        (WHITESPACE, 1),
                        BINARY_EXP => {
                            (IDENTIFIER, 1),
                            (WHITESPACE, 1),
                            (GREATER, 1),
                            (WHITESPACE, 1),
                            (FLOAT_LITERAL, 4),
                            (WHITESPACE, 1),
                        },
                    },
                    (OR_KW, 2),
                    (WHITESPACE, 1),
                    BINARY_EXP => {
                        BINARY_EXP => {
                            (INTEGER_LITERAL, 2),
                            (WHITESPACE, 1),
                            (PERCENT, 1),
                            (WHITESPACE, 1),
                            (INTEGER_LITERAL, 1),
                            (WHITESPACE, 1),
                        },
                        (EQUAL, 2),
                        (WHITESPACE, 1),
                        (INTEGER_LITERAL, 1),
                        (WHITESPACE, 1),
                    },
                },
                (FAT_ARROW, 2),
                (WHITESPACE, 1),
                FUNCTION_APPLICATION => {
                    MEMBER_REFERENCE => {
                        (IDENTIFIER, 6),
                        (DOT, 1),
                        (IDENTIFIER, 6),
                    },
                    (LEFT_PAREN, 1),
                    ARGUMENT_LIST => {
                        POSITIONAL_ARGUMENT => {
                            ELEMENT_ACCESS => {
                                (IDENTIFIER, 1),
                                (LEFT_BRACKET, 1),
                                (IDENTIFIER, 1),
                                (RIGHT_BRACKET, 1),
                            },
                            (COMMA, 1),
                            (WHITESPACE, 1),
                        },
                        NAMED_ARGUMENT => {
                            (IDENTIFIER, 1),
                            (COLON, 1),
                            (WHITESPACE, 1),
                            BINARY_EXP => {
                                (INTEGER_LITERAL, 1),
                                (WHITESPACE, 1),
                                (STAR, 1),
                                (WHITESPACE, 1),
                                (IDENTIFIER, 2),
                            },
                        },
                    },
                    (RIGHT_PAREN, 1),
                },
            },
            EOF,
        };

        let mut p = Parser::new(source);
        parse_expr(&mut p);
        let (diagnostics, tree) = p.finish();

        assert!(diagnostics.is_empty());
        assert_eq!(tree, expected);

        Ok(())
    }

    #[test]
    fn test_parse_osc_file() -> Result<(), Box<dyn std::error::Error>> {
        println!("test_parse_osc_file");
        let source = r#"
scenario sut.my__scenario:
    car1: vehicle
    car2: vehicle

    do serial:
        phase1: car1.drive(duration: 24s) with:
            speed([40kph..80kph], at: end)
            lane([2..4])
        phase2: car1.drive(duration: 24s) with:
            speed([70kph..60kph], at: end)
"#;

        let mut p = Parser::new(source);
        parse_osc_file(&mut p);
        let (diagnostics, tree) = p.finish();

        let mut pretty = Vec::new();
        syntree::print::print_with_source(&mut pretty, &tree, &source)?;
        let pretty = String::from_utf8(pretty)?;
        println!("{}", pretty);
        Ok(())
    }
}
