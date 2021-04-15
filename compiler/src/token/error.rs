use crate::error::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum ScanError {
    UnterminatedString { start: usize },
    InvalidCharacter { position: usize },
    // TODO: Rust doesn't allow you to inspect the cause yet
    InvalidInteger { span: Span },
}

impl From<ScanError> for crate::error::Citation {
    fn from(error: ScanError) -> Self {
        use crate::error::Citation;
        match error {
            ScanError::UnterminatedString { start } => {
                Citation::error("Unterminated string".to_owned()).span(
                    Span { start, end: start },
                    Some("string starts here".to_owned()),
                )
            }
            ScanError::InvalidCharacter { position } => {
                Citation::error("Invalid character".to_owned()).span(
                    Span {
                        start: position,
                        end: position,
                    },
                    None,
                )
            }
            ScanError::InvalidInteger { span } => {
                Citation::error("Invalid integer".to_owned()).span(span, None)
            }
        }
    }
}
