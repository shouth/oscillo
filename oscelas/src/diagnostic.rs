use std::{fmt::Display, ops::Range};
use std::fmt::Write;

use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::syntax::{OscSyntaxKind::{self, *}, OscSyntaxKindSet};

trait DiagnosticDisplay: Sized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    fn display(&self) -> DiagnosticDisplayRef<Self> {
        DiagnosticDisplayRef(self)
    }
}

struct DiagnosticDisplayRef<'a, T>(&'a T);

impl<'a, T> Display for DiagnosticDisplayRef<'a, T>
where
    T: DiagnosticDisplay,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DiagnosticDisplay::fmt(self.0, f)
    }
}

impl DiagnosticDisplay for OscSyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.static_token() {
            Some(token) => write!(f, "`{token}`")?,
            None => match self {
                IDENTIFIER => write!(f, "identifier")?,
                INTEGER_LITERAL => write!(f, "integer literal")?,
                FLOAT_LITERAL => write!(f, "float literal")?,
                STRING_LITERAL => write!(f, "string literal")?,
                _ => write!(f, "{:?}", self)?,
            }
        }
        Ok(())
    }
}

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
        found: OscSyntaxKind,
    },
    ExpectedExpression {
        range: Range<usize>,
        found: OscSyntaxKind,
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
            SyntaxDiagnostic::UnexpectedToken { range, expected, found } => {
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
                    write!(&mut message, "{}", kind.display()).unwrap();
                }
                write!(&mut message, ", found {}", found.display()).unwrap();

                Diagnostic::error()
                    .with_message(message)
                    .with_labels(vec![
                        Label::primary(file_id.clone(), range)
                    ])
            }
            SyntaxDiagnostic::ExpectedExpression { range, found } => {
                Diagnostic::error()
                    .with_message(format!("expected expression, found {}", found.display()))
                    .with_labels(vec![
                        Label::primary(file_id.clone(), range)
                    ])
            }
        }
    }
}
