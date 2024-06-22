mod common;
mod decl;
mod expr;
mod member;
mod osc_file;

use std::collections::HashSet;
use std::fmt::Write;

use syntree::pointer::PointerUsize;
use syntree::{Builder as TreeBuilder, Checkpoint as TreeCheckpoint, Tree};

use crate::diagnostic::Diagnostic;
use crate::lexer::{LexicalAnalyzer, Lookahead};
use crate::syntax::OscDslSyntaxKind::{self, *};

#[derive(Debug, Clone)]
pub struct Checkpoint(TreeCheckpoint<PointerUsize>);

pub struct Parser<'a> {
    source: &'a str,
    lexer: Lookahead<LexicalAnalyzer<'a>>,
    builder: TreeBuilder<OscDslSyntaxKind, u32, usize>,
    diagnostic: Vec<Diagnostic>,
    expected: HashSet<OscDslSyntaxKind>,
    leading_trivia: bool,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        let mut parser = Parser {
            source,
            lexer: Lookahead::new(LexicalAnalyzer::new(source)),
            builder: TreeBuilder::new(),
            diagnostic: Vec::new(),
            expected: HashSet::new(),
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

    fn has_leading_trivia(&self) -> bool {
        self.leading_trivia
    }

    pub fn bump(&mut self, kind: OscDslSyntaxKind) {
        let token = self.lexer.bump();
        self.builder.token(kind, token.length).unwrap();
        self.expected.clear();
        self.skip_trivia();
    }

    pub fn check_any(&mut self, kinds: &[OscDslSyntaxKind]) -> bool {
        self.expected.extend(kinds.iter());

        let next = self.lexer.nth(0).kind;

        if next == IDENTIFIER {
            for kind in kinds {
                if let Some(token) = kind.static_token() {
                    let begin = self.lexer.offset();
                    let length = self.lexer.nth(0).length;
                    let lexeme = &self.source[begin..][..length];
                    if lexeme == token {
                        return true;
                    }
                }
            }
        }

        kinds.iter().any(|&kind| kind == next)
    }

    pub fn check(&mut self, kind: OscDslSyntaxKind) -> bool {
        self.check_any(&[kind])
    }

    pub fn eat_any(&mut self, kinds: &[OscDslSyntaxKind]) -> bool {
        for kind in kinds {
            if self.check(*kind) {
                self.bump(*kind);
                return true;
            }
        }
        false
    }

    pub fn eat(&mut self, kind: OscDslSyntaxKind) -> bool {
        self.eat_any(&[kind])
    }

    pub fn expect_any(&mut self, kinds: &[OscDslSyntaxKind]) -> bool {
        for kind in kinds {
            if self.eat(*kind) {
                return true;
            }
        }
        self.unexpected();
        false
    }

    pub fn expect(&mut self, kind: OscDslSyntaxKind) -> bool {
        self.expect_any(&[kind])
    }

    fn unexpected(&mut self) {
        let mut expected = Vec::from_iter(&self.expected);
        expected.sort();
        let mut message = String::new();
        write!(&mut message, "expected ").unwrap();
        if expected.len() > 1 {
            write!(&mut message, "one of ").unwrap();
        }
        for (i, kind) in expected.iter().enumerate() {
            if i > 0 {
                write!(&mut message, ", ").unwrap();
            }
            write!(&mut message, "{:?},", kind).unwrap();
        }
        write!(&mut message, ", found {:?},", self.lexer.nth(0).kind).unwrap();

        let start = self.lexer.offset();
        let end = start + self.lexer.nth(0).length;
        self.diagnostic.push(Diagnostic::new(start..end, message));
    }

    pub fn open(&mut self) -> Checkpoint {
        Checkpoint(self.builder.checkpoint().unwrap())
    }

    pub fn close(&mut self, checkpoint: Checkpoint, kind: OscDslSyntaxKind) {
        self.builder.close_at(&checkpoint.0, kind).unwrap();
    }

    pub fn finish(mut self) -> (Vec<Diagnostic>, Tree<OscDslSyntaxKind, u32, usize>) {
        loop {
            self.skip_trivia();
            let token = self.lexer.bump();
            if token.kind == EOF {
                self.builder.token(EOF, 0).unwrap();
                break;
            }
            self.builder.token(ERROR, token.length).unwrap();
        }
        (self.diagnostic, self.builder.build().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use expr::parse_expr;

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
}
