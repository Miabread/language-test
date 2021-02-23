use colored::Colorize;

/// A Span in some source code.
#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

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

pub fn report(source: &str, errors: Vec<impl Report>) {
    let mut fmt = ReportFormatter { source };
    for error in errors {
        error.report(&mut fmt);
    }
}

pub trait Report {
    fn report(&self, fmt: &mut ReportFormatter<'_>);
}

#[derive(Debug, Clone)]
pub struct ReportFormatter<'src> {
    source: &'src str,
}

impl<'src> ReportFormatter<'src> {
    pub fn error(&self, message: &str) {
        eprintln!("{} {}", "error:".red().bold(), message.bold());
    }

    pub fn source_end(&self) -> usize {
        self.source.len() - 1
    }

    pub fn span(&self, span: Span, message: &str) {
        let span = self.resolve(span);
        if span.line_number_start == span.line_number_end {
            self.single_line(span, message);
        } else {
            self.multi_line(span, message);
        }
    }

    fn single_line(&self, span: ResolvedSpan, message: &str) {
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
            message.red(),
        );
    }

    fn multi_line(&self, span: ResolvedSpan, message: &str) {
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
                    message.red(),
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

    fn resolve(&self, span: Span) -> ResolvedSpan {
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

/// Iterator yielding every line in a string. The line includes newline character(s).
pub struct LinesWithEndings<'a> {
    input: &'a str,
}

impl<'a> LinesWithEndings<'a> {
    pub fn from(input: &'a str) -> LinesWithEndings<'a> {
        LinesWithEndings { input }
    }
}

impl<'a> Iterator for LinesWithEndings<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        if self.input.is_empty() {
            return None;
        }
        let split = self
            .input
            .find('\n')
            .map(|i| i + 1)
            .unwrap_or_else(|| self.input.len());
        let (line, rest) = self.input.split_at(split);
        self.input = rest;
        Some(line)
    }
}
