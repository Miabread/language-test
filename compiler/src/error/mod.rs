mod reporter;
mod util;

use colored::Colorize;
pub use reporter::Reporter;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A Span in some source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Debug, Clone)]
enum Level {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
struct SpanLabel {
    span: Span,
    message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Citation {
    message: String,
    level: Level,
    spans: Vec<SpanLabel>,
}

impl Citation {
    pub fn error(message: String) -> Self {
        Self {
            message,
            level: Level::Error,
            spans: Vec::new(),
        }
    }

    pub fn warning(message: String) -> Self {
        Self {
            message,
            level: Level::Warning,
            spans: Vec::new(),
        }
    }

    pub fn info(message: String) -> Self {
        Self {
            message,
            level: Level::Info,
            spans: Vec::new(),
        }
    }

    pub fn span(mut self, span: Span, message: Option<String>) -> Self {
        self.spans.push(SpanLabel { span, message });
        self
    }

    pub fn push_span(&mut self, span: Span, message: Option<String>) -> &mut Self {
        self.spans.push(SpanLabel { span, message });
        self
    }
}
