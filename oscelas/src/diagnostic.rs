use std::ops::Range;
use std::fmt::Write;

use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::syntax::{OscSyntaxKind::{self, *}, OscSyntaxKindSet};

pub enum SyntaxDiagnostic {
    StrayCharacter {
        range: Range<usize>,
        character: char,
    },
    IrregularIndentationSize {
        range: Range<usize>,
        expected: usize,
        actual: usize,
    },
    IrregularIndentationSequence {
        range: Range<usize>,
        previous: Range<usize>,
    },
    UnexpectedToken {
        range: Range<usize>,
        expected: OscSyntaxKindSet,
        actual: OscSyntaxKind,
    },
}

impl SyntaxDiagnostic {
    pub fn into_codespan_reporting<FileId: Clone>(self, file_id: &FileId) -> Diagnostic<FileId> {
        match self {
            SyntaxDiagnostic::StrayCharacter { range, character } => {
                Diagnostic::error()
                    .with_message(format!("stray `{}` in program", character))
                    .with_labels(vec![
                        Label::primary(file_id.clone(), range)
                    ])
            }
            SyntaxDiagnostic::IrregularIndentationSize { range, expected, actual } => {
                Diagnostic::error()
                    .with_message(format!("expected indentation of width {}, found {}", expected, actual))
                    .with_labels(vec![
                        Label::primary(file_id.clone(), range)
                    ])
            }
            SyntaxDiagnostic::IrregularIndentationSequence { range, previous } => {
                Diagnostic::error()
                    .with_message(format!("indentation characters are inconsistent"))
                    .with_labels(vec![
                        Label::primary(file_id.clone(), range)
                            .with_message("inconsistent indentation"),
                        Label::secondary(file_id.clone(), previous)
                            .with_message("previous indentation"),
                    ])
            }
            SyntaxDiagnostic::UnexpectedToken { range, expected, actual } => {
                let mut message = String::new();
                write!(&mut message, "expected ").unwrap();
                if expected.len() > 1 {
                    write!(&mut message, "one of ").unwrap();
                }
                for (i, kind) in expected.iter().enumerate() {
                    if i > 0 {
                        if i + 1 < expected.len() {
                            write!(&mut message, ", ").unwrap();
                        } else {
                            write!(&mut message, " and ").unwrap();
                        }
                    }

                    let display_name = match kind.static_token() {
                        Some(token) => token,
                        None => match kind {
                            EOF => "EOF",
                            INTEGER_LITERAL => "`integer literal`",
                            FLOAT_LITERAL => "`float literal`",
                            STRING_LITERAL => "`string literal`",
                            NEWLINE => "NEWLINE",
                            INDENT => "INDENT",
                            DEDENT => "DEDENT",
                            IDENTIFIER => "`identifier`",
                            _ => unreachable!(),
                        }
                    };
                    write!(&mut message, "{}", display_name).unwrap();
                }
                write!(&mut message, ", found {:?}", actual).unwrap();

                Diagnostic::error()
                    .with_message(message)
                    .with_labels(vec![
                        Label::primary(file_id.clone(), range)
                    ])
            }
        }
    }
}
