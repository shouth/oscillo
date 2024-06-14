use std::ops::Range;

pub struct Diagnostic {
    span: Range<usize>,
    message: String,
    labels: Vec<Label>,
}

pub struct Label {
    span: Range<usize>,
    message: String,
}

impl Diagnostic {
    pub fn new(span: impl Into<Range<usize>>, message: impl ToString) -> Diagnostic {
        Diagnostic {
            span: span.into(),
            message: message.to_string(),
            labels: Vec::new(),
        }
    }

    pub fn with_labels(mut self, labels: Vec<Label>) -> Diagnostic {
        self.labels = labels;
        self
    }

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn labels(&self) -> &[Label] {
        &self.labels
    }
}

impl Label {
    pub fn new(span: Range<usize>) -> Label {
        Label {
            span,
            message: String::new(),
        }
    }

    pub fn with_message(mut self, message: String) -> Label {
        self.message = message;
        self
    }

    pub fn span(&self) -> Range<usize> {
        self.span.clone()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
