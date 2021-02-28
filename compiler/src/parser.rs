use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Debug(Box<Expression>),
}

pub fn parse<'src>(
    mut source: impl Iterator<Item = Token<'src>>,
) -> Result<Expression, ParseError<'src>> {
    let token = source.next().unwrap();
    Ok(match token.kind {
        TokenKind::Number(num) => Expression::Number(num),
        TokenKind::DebugKeyword => Expression::Debug(Box::new(parse(source)?)),
        _ => return Err(ParseError::UnimplementedToken { token }),
    })
}

#[derive(Debug, Clone)]
pub enum ParseError<'src> {
    UnimplementedToken { token: Token<'src> },
}

impl<'src> From<ParseError<'src>> for crate::error::Citation {
    fn from(error: ParseError<'src>) -> Self {
        use crate::error::Citation;
        match error {
            ParseError::UnimplementedToken { token } => {
                Citation::error("Unimplemented token type".to_owned()).span(token.span, None)
            }
        }
    }
}
