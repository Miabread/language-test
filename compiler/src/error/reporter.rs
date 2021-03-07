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
        let side_width = span.line_number_end.to_string().len();

        // Start padding
        self.blank_side(side_width);
        self.new_line();

        // Source line
        self.line_number_side(span.line_number_start, side_width);
        self.line(line);
        self.new_line();

        // Label line
        self.blank_side(side_width);
        self.repeat(" ", span.local_start + 1);
        // Columns are zero indexed, must +1 for correct alignment
        self.repeat("^".red().bold(), span.local_end - span.local_start + 1);
        self.message(&label.message);
        self.new_line();

        // End padding
        self.blank_side(side_width);
        self.new_line();
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
        let side_width = span.line_number_end.to_string().len();

        for (line_number, line) in lines {
            if line_number == span.line_number_start {
                // Start padding
                self.blank_side(side_width);
                self.new_line();

                // Source line
                self.line_number_side(line_number, side_width);
                self.error_bar_align();
                self.line(line);
                self.new_line();

                // Start label line
                self.blank_side(side_width);
                self.error_bar_align();
                self.repeat("_".bold().red(), span.local_start + 1);
                eprint!("{}", "^".bold().red());
                self.new_line();
            } else if line_number == span.line_number_end {
                // Source line
                self.line_number_side(line_number, side_width);
                self.error_bar();
                self.line(line);
                self.new_line();

                // End label line
                self.blank_side(side_width);
                self.error_bar();
                self.repeat("_".bold().red(), span.local_end + 1);
                eprint!("{}", "^".bold().red());
                self.message(&label.message);
                self.new_line();

                // End padding
                self.blank_side(side_width);
                self.new_line();
            } else {
                // Source line
                self.line_number_side(line_number, side_width);
                self.error_bar();
                self.line(line);
                self.new_line();
            }
        }
    }

    fn new_line(&self) {
        eprintln!();
    }

    fn line_number_side(&self, line_number: usize, side_width: usize) {
        eprint!(
            "{}",
            format!("{:1$} |", line_number, side_width).cyan().bold()
        );
    }

    fn blank_side(&self, side_width: usize) {
        eprint!("{}", format!("{:1$} |", "", side_width).cyan().bold());
    }

    fn line(&self, line: &str) {
        eprint!(" {}", line);
    }

    fn error_bar_align(&self) {
        eprint!("  ");
    }

    fn error_bar(&self) {
        eprint!("{}", " |".red().bold());
    }

    fn repeat(&self, text: impl std::fmt::Display, times: usize) {
        for _ in 0..times {
            eprint!("{}", text);
        }
    }

    fn message(&self, message: &Option<String>) {
        if let Some(msg) = message {
            eprint!("{}", msg.red().bold())
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
