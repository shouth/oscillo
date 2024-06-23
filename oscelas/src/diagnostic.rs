use std::ops::Range;

use crate::syntax::{OscSyntaxKind, OscSyntaxKindSet};

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
        expected: String,
        actual: String,
    },
    UnexpectedToken {
        range: Range<usize>,
        expected: OscSyntaxKindSet,
        actual: OscSyntaxKind,
    },
}
