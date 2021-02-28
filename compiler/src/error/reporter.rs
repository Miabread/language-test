use super::{util::LinesWithEndings, *};

/// A Span that has more line information resolved. Used in error reporting.
#[derive(Debug, Clone)]
struct ResolvedSpan {
    source_start: usize,
    source_end: usize,
    line_number_start: usize,
    line_number_end: usize,
    local_start: usize,
    local_end: usize,
}

#[derive(Debug, Clone)]
pub struct Reporter<'src> {
    source: &'src str,
}

impl<'src> Reporter<'src> {
    pub fn new(source: &'src str) -> Self {
        Reporter { source }
    }

    pub fn report(&self, citations: &[Citation]) {
        for citation in citations {
            self.citation(citation);
            for span in &citation.spans {
                self.span(span);
            }
        }
    }

    fn citation(&self, citation: &Citation) {
        eprintln!(
            "{} {}",
            match citation.level {
                Level::Error => "error:".red().bold(),
                Level::Warning => "warning:".yellow().bold(),
                Level::Info => "info:".blue().bold(),
            },
            citation.message,
        );
    }

    fn span(&self, label: &SpanLabel) {
        let span = &self.resolve(&label.span);
        if span.line_number_start == span.line_number_end {
            self.single_line(label, span);
        } else {
            self.multi_line(label, span);
        }
    }

    fn single_line(&self, label: &SpanLabel, span: &ResolvedSpan) {
        let line = self.source.lines().nth(span.line_number_start - 1).unwrap();

        // Needed to pad all numbers to same length
        let number_width = span.line_number_end.to_string().len();

        // Source code line
        eprintln!(
            "{} {}",
            // Line number + side bar
            format!("{:1$} |", span.line_number_start, number_width)
                .cyan()
                .bold(),
            line,
        );

        // Label line
        eprintln!(
            "{} {}{} {}",
            // Sidebar
            format!("{:1$} |", "", number_width).cyan().bold(),
            // Pad the label
            " ".repeat(span.local_start),
            // Label the source, label must have max of 1 for when span is single char
            "^".repeat((span.local_end - span.local_start).max(1)).red(),
            if let Some(msg) = &label.message {
                msg.red()
            } else {
                "".red()
            }
        );
    }

    fn multi_line(&self, label: &SpanLabel, span: &ResolvedSpan) {
        let lines = self
            .source
            .lines()
            // Take all lines contained the in the span
            .skip(span.line_number_start - 1)
            .take(span.line_number_end - (span.line_number_start - 1))
            // Pair them with their line numbers
            .enumerate()
            .map(|(i, line)| (i + span.line_number_start, line))
            .peekable();

        // Needed to pad all numbers to same length
        let number_width = span.line_number_end.to_string().len();

        for (num, line) in lines {
            // When first line
            if num == span.line_number_start {
                eprintln!(
                    "{}   {}",
                    // Line number + side bar
                    format!("{:1$} |", span.line_number_start, number_width)
                        .cyan()
                        .bold(),
                    line,
                );
                eprintln!(
                    "{}  {}",
                    // Side bar
                    format!("{:1$} |", "", number_width).cyan().bold(),
                    format!("{}^", "_".repeat(span.local_start + 1)).red(),
                );
            }
            // When last line
            else if num == span.line_number_end {
                eprintln!(
                    "{} {} {}",
                    // Line number + side bar
                    format!("{:1$} |", span.line_number_end, number_width)
                        .cyan()
                        .bold(),
                    // Error bar
                    "|".red(),
                    line,
                );
                eprintln!(
                    "{} {} {}",
                    // Side bar
                    format!("{:1$} |", "", number_width).cyan().bold(),
                    // Error bar + label
                    format!("|{}^", "_".repeat(span.local_end + 1)).red(),
                    if let Some(msg) = &label.message {
                        msg.red()
                    } else {
                        "".red()
                    },
                );
            }
            // When in between line
            else {
                eprintln!(
                    "{} {} {}",
                    // Line number + side bar
                    format!("{:1$} |", num, number_width).cyan().bold(),
                    // Error bar
                    "|".red(),
                    line,
                );
            }
        }
    }

    fn resolve(&self, span: &Span) -> ResolvedSpan {
        // Count lines before the start/end
        let line_number_start = self.source[..=span.start].lines().count();
        let line_number_end = self.source[..=span.end].lines().count();

        // Find the index of the start of the lines by counting all characters up to it
        let line_index_start: usize = LinesWithEndings::from(self.source)
            .take(line_number_start - 1)
            .map(|l| l.len())
            .sum();
        let line_index_end: usize = LinesWithEndings::from(self.source)
            .take(line_number_end - 1)
            .map(|l| l.len())
            .sum();

        ResolvedSpan {
            // Same as the normal span
            source_start: span.start,
            source_end: span.end,
            // Computed above
            line_number_start,
            line_number_end,
            // Find the local ends by subtracting the line offset
            local_start: span.start - line_index_start,
            local_end: span.end - line_index_end,
        }
    }
}
