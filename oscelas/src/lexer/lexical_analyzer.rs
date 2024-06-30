use crate::diagnostic::SyntaxDiagnostic;
use crate::lexer::LexedToken;
use crate::lexer::lexer::Lexer;
use crate::syntax::OscSyntaxKind::{self, *};

#[derive(Debug)]
pub struct Indentation<'a> {
    pub text: &'a str,
    pub offset: usize,
    pub width: usize,
}

impl Indentation<'_> {
    pub fn new<'a>(text: &'a str, offset: usize) -> Indentation<'a> {
        let text_width = text[offset..]
            .chars()
            .take_while(|c| [' ', '\t'].contains(c))
            .count();

        let text = &text[offset..][..text_width];
        let width = text
            .chars()
            .map(|c| if c == ' ' { 1 } else { 8 })
            .sum();

        Indentation { text, offset, width }
    }
}

enum LexicalAnalyzerState {
    StartOfLine,
    Normal,
    EndOfLine,
}

pub struct LexicalAnalyzer<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    next: [LexedToken; 2],
    indentations: Vec<Indentation<'a>>,
    nesting: i32,
    diagnostics: Vec<SyntaxDiagnostic>,
    state: LexicalAnalyzerState,
}

impl<'a> LexicalAnalyzer<'a> {
    pub fn new(source: &'a str) -> LexicalAnalyzer<'a> {
        let mut lexer = Lexer::new(source);
        let next = [lexer.next_token(), lexer.next_token()];

        LexicalAnalyzer {
            source,
            lexer,
            next,
            indentations: vec![Indentation::new(source, 0)],
            nesting: 0,
            diagnostics: Vec::new(),
            state: LexicalAnalyzerState::StartOfLine,
        }
    }

    pub fn offset(&self) -> usize {
        self.lexer.offset() - self.next[0].length - self.next[1].length
    }

    pub fn left(&mut self) {
        self.nesting += 1;
    }

    pub fn right(&mut self) {
        assert!(self.nesting > 0);
        self.nesting -= 1;
    }

    fn bump(&mut self) -> LexedToken {
        let current = self.next[0];
        self.next = [self.next[1], self.lexer.next_token()];
        current
    }

    fn spawn(&self, kind: OscSyntaxKind) -> LexedToken {
        LexedToken { kind, length: 0 }
    }

    pub fn next_token(&mut self) -> LexedToken {
        match self.state {
            LexicalAnalyzerState::StartOfLine => match self.next[0].kind {
                TRIVIAL_NEWLINE => {
                    self.bump()
                }
                WHITESPACE | EOF => {
                    if self.next[1].kind == TRIVIAL_NEWLINE {
                        return self.bump();
                    }

                    let indentation = Indentation::new(&self.source, self.offset());
                    let last_indentation = self.indentations.last()
                        .expect("indentations should not be empty");

                    self.state = LexicalAnalyzerState::Normal;
                    if indentation.width > last_indentation.width {
                        self.indentations.push(indentation);
                        self.spawn(INDENT)
                    } else if indentation.width < last_indentation.width {
                        let last_indentation = self.indentations.pop()
                            .expect("indentations should not be empty");
                        let outer_indentation = self.indentations.last()
                            .expect("indentations should not be empty");

                        if indentation.width < outer_indentation.width {
                            self.state = LexicalAnalyzerState::StartOfLine;
                        }

                        if indentation.width > outer_indentation.width {
                            let start = self.offset();
                            let end = start + self.next[0].length;
                            self.diagnostics.push(SyntaxDiagnostic::IrregularIndentationSize {
                                range: start..end,
                                expected: outer_indentation.width,
                                actual: indentation.width,
                            });

                            self.indentations.push(last_indentation);
                            self.bump()
                        } else {
                            self.spawn(DEDENT)
                        }
                    } else {
                        if indentation.text != last_indentation.text {
                            let start = self.offset();
                            let end = start + self.next[0].length;
                            let last_start = last_indentation.offset;
                            let last_end = last_start + last_indentation.text.len();
                            self.diagnostics.push(SyntaxDiagnostic::IrregularIndentationSequence {
                                range: start..end,
                                last: last_start..last_end,
                            });
                        }
                        self.bump()
                    }
                }
                _ => {
                    self.state = LexicalAnalyzerState::Normal;
                    self.bump()
                }
            }
            LexicalAnalyzerState::Normal => match self.next[0].kind {
                TRIVIAL_NEWLINE => {
                    if self.nesting == 0 {
                        self.state = LexicalAnalyzerState::EndOfLine;
                    }
                    self.bump()
                }
                EOF => {
                    if self.indentations.len() > 1 {
                        self.indentations.pop();
                        self.spawn(DEDENT)
                    } else {
                        self.bump()
                    }
                }
                _ => self.bump(),
            }
            LexicalAnalyzerState::EndOfLine => {
                self.state = LexicalAnalyzerState::StartOfLine;
                self.spawn(NEWLINE)
            }
        }
    }

    pub fn finish(self) -> Vec<SyntaxDiagnostic> {
        assert!(self.nesting == 0);
        let mut diagnostics = self.diagnostics;
        diagnostics.extend(self.lexer.finish());
        diagnostics
    }
}
