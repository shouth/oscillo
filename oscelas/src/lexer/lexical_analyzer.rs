use crate::diagnostic::SyntaxDiagnostic;
use crate::syntax::OscSyntaxKind::{self, *};

use super::lookahead::{Lookahead, LookaheadSource};
use super::LexedToken;
use super::lexer::Lexer;

pub struct Indentation<'a> {
    pub text: &'a str,
    pub offset: usize,
    pub width: usize,
}

impl<'a> Indentation<'a> {
    pub fn new(text: &str, offset: usize) -> Indentation {
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
    lexer: Lookahead<Lexer<'a>>,
    indentations: Vec<Indentation<'a>>,
    enclosing_level: i32,
    diagnostics: Vec<SyntaxDiagnostic>,
    state: LexicalAnalyzerState,
}

impl<'a> LexicalAnalyzer<'a> {
    pub fn new(source: &'a str) -> LexicalAnalyzer<'a> {
        LexicalAnalyzer {
            source,
            lexer: Lookahead::new(Lexer::new(source)),
            indentations: vec![Indentation::new("", 0)],
            enclosing_level: 0,
            diagnostics: Vec::new(),
            state: LexicalAnalyzerState::StartOfLine,
        }
    }

    pub fn push_enclosure(&mut self) {
        self.enclosing_level += 1;
    }

    pub fn pop_enclosure(&mut self) {
        assert!(self.enclosing_level > 0);
        self.enclosing_level -= 1;
    }

    fn spawn(&self, kind: OscSyntaxKind) -> LexedToken {
        LexedToken { kind, length: 0 }
    }

    pub fn next_token(&mut self) -> LexedToken {
        match self.state {
            LexicalAnalyzerState::StartOfLine => match self.lexer.nth(0).kind {
                TRIVIAL_NEWLINE => {
                    self.lexer.bump()
                }
                WHITESPACE => {
                    if self.lexer.nth(1).kind == TRIVIAL_NEWLINE {
                        return self.lexer.bump();
                    }

                    let indentation = Indentation::new(&self.source, self.lexer.offset());
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

                        if indentation.width > outer_indentation.width {
                            let start = self.lexer.offset();
                            let end = start + self.lexer.nth(0).length;
                            self.diagnostics.push(SyntaxDiagnostic::IrregularIndentationSize {
                                range: start..end,
                                expected: outer_indentation.width,
                                actual: indentation.width,
                            });

                            self.indentations.push(last_indentation);
                            self.lexer.bump()
                        } else {
                            self.spawn(DEDENT)
                        }
                    } else {
                        if indentation.text != last_indentation.text {
                            let start = self.lexer.offset();
                            let end = start + self.lexer.nth(0).length;
                            let last_start = last_indentation.offset;
                            let last_end = last_start + last_indentation.text.len();
                            self.diagnostics.push(SyntaxDiagnostic::IrregularIndentationSequence {
                                range: start..end,
                                last: last_start..last_end,
                            });
                        }
                        self.lexer.bump()
                    }
                }
                _ => {
                    self.state = LexicalAnalyzerState::Normal;
                    self.lexer.bump()
                }
            }
            LexicalAnalyzerState::Normal => match self.lexer.nth(0).kind {
                TRIVIAL_NEWLINE => {
                    if self.enclosing_level == 0 {
                        self.state = LexicalAnalyzerState::EndOfLine;
                    }
                    self.lexer.bump()
                }
                _ => self.lexer.bump(),
            }
            LexicalAnalyzerState::EndOfLine => {
                self.state = LexicalAnalyzerState::StartOfLine;
                self.spawn(NEWLINE)
            }
        }
    }

    pub fn finish(self) -> Vec<SyntaxDiagnostic> {
        let mut diagnostics = self.diagnostics;
        diagnostics.extend(self.lexer.finish().finish());
        diagnostics
    }
}

impl LookaheadSource for LexicalAnalyzer<'_> {
    fn next_token(&mut self) -> LexedToken {
        self.next_token()
    }
}
