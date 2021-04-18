use crate::error::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum ScanError {
    InvalidToken { position: usize },
    // TODO: Rust doesn't allow you to inspect the cause yet
    InvalidInteger { span: Span },
    UnterminatedCharacterEof { start: usize },
    EmptyCharacter { span: Span },
    CharacterExpectedClosing { actual: char, span: Span },
    UnterminatedString { start: usize },
}

impl From<ScanError> for crate::error::Citation {
    fn from(error: ScanError) -> Self {
        use crate::error::Citation;
        match error {
            ScanError::InvalidToken { position } => Citation::error("Invalid token".to_owned())
                .span(
                    Span {
                        start: position,
                        end: position,
                    },
                    None,
                ),
            ScanError::InvalidInteger { span } => {
                Citation::error("Invalid integer literal".to_owned()).span(span, None)
            }
            ScanError::UnterminatedCharacterEof { start } => {
                Citation::error("Rest of character literal expected, found end of file".to_owned())
                    .span(
                        Span { start, end: start },
                        Some("character starts here".to_owned()),
                    )
            }
            ScanError::EmptyCharacter { span: position } => {
                Citation::error("Empty character literal, expected character".to_owned())
                    .span(position, None)
            }
            ScanError::CharacterExpectedClosing {
                actual,
                span: position,
            } => Citation::error(format!(
                "Character literal expected closing quote, found {}",
                actual
            ))
            .span(position, None),
            ScanError::UnterminatedString { start } => {
                Citation::error("Unterminated string literal".to_owned()).span(
                    Span { start, end: start },
                    Some("string starts here".to_owned()),
                )
            }
        }
    }
}
