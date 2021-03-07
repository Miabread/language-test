use crate::token::{Token, TokenKind};

pub fn parse<'src>(
    mut source: impl Iterator<Item = Token<'src>>,
) -> Result<Vec<f64>, Vec<ParseError<'src>>> {
    let mut result = Vec::new();
    let mut errors = Vec::new();

    while let Some(token) = source.next() {
        match token.kind {
            TokenKind::Number(number) => result.push(number),
            TokenKind::Semicolon => continue,
            _ => {
                errors.push(ParseError::Unimplemented { token });
                continue;
            }
        }

        if let Some(token) = source.next() {
            if let TokenKind::Semicolon = token.kind {
                continue;
            } else {
                errors.push(ParseError::ExpectedSemicolon { token });
                continue;
            }
        } else {
            break;
        };
    }

    if errors.is_empty() {
        Ok(result)
    } else {
        Err(errors)
    }
}

#[derive(Debug, Clone)]
pub enum ParseError<'src> {
    ExpectedSemicolon { token: Token<'src> },
    Unimplemented { token: Token<'src> },
}

impl<'src> From<ParseError<'src>> for crate::error::Citation {
    fn from(error: ParseError<'src>) -> Self {
        use crate::error::Citation;
        match error {
            ParseError::ExpectedSemicolon { token } => {
                Citation::error("Expected semicolon".to_owned()).span(token.span, None)
            }
            ParseError::Unimplemented { token } => {
                Citation::error("Unimplemented".to_owned()).span(token.span, None)
            }
        }
    }
}
