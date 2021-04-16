mod data;
mod error;
#[cfg(test)]
mod test;

use crate::error::Span;
pub use data::{Keyword, Token, TokenKind};
pub use error::ScanError;
use itertools::Itertools;
use std::{
    iter::Peekable,
    str::{CharIndices, FromStr},
};

pub fn scan(source: &str) -> (Vec<Token<'_>>, Vec<ScanError>) {
    Scanner::new(source).scan()
}

struct Scanner<'src> {
    source: &'src str,
    chars: Peekable<CharIndices<'src>>,
}

impl<'src> Scanner<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            chars: source.char_indices().peekable(),
        }
    }

    fn scan(&mut self) -> (Vec<Token<'src>>, Vec<ScanError>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while let Some(eee) = self.next() {
            match eee {
                Ok(token) => tokens.push(token),
                Err(error) => errors.push(error),
            }
        }

        (tokens, errors)
    }

    fn next(&mut self) -> Option<Result<Token<'src>, ScanError>> {
        let head = self.trim()?;

        let kind = match head.1 {
            // Single char tokens get handled outside the match
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,

            // Parse number literals
            number if number.is_ascii_digit() => {
                // Keep consuming digit chars, last digit needed for slice
                let last = self
                    .chars
                    .peeking_take_while(|it| it.1.is_ascii_digit())
                    .last()
                    .unwrap_or(head);

                // TODO: Implement float literals. Delayed due to needing double look ahead.

                let span = Span {
                    start: head.0,
                    end: last.0,
                };

                // Parse the digits into a number
                return Some(
                    self.source[head.0..=last.0]
                        .parse()
                        .map(TokenKind::Integer)
                        .map(|kind| Token { span, kind })
                        .map_err(|_| ScanError::InvalidInteger { span }),
                );
            }

            // Parse string literals
            '"' => {
                // Keep consuming chars until a quote
                self.chars
                    .peeking_take_while(|it| it.1 != '"')
                    .for_each(drop);

                // Expect a closing quote
                let closing = if let Some(closing) = self.chars.next_if(|it| it.1 == '"') {
                    closing
                } else {
                    return Some(Err(ScanError::UnterminatedString { start: head.0 }));
                };

                // Slice the leading and trailing quote
                let kind = TokenKind::String(&self.source[head.0 + 1..closing.0]);

                return Some(Ok(Token {
                    span: Span {
                        start: head.0,
                        end: closing.0,
                    },
                    kind,
                }));
            }

            letter if letter.is_alphabetic() => {
                // Keep consuming chars, last char needed for slice
                let last = self
                    .chars
                    .peeking_take_while(|it| it.1.is_alphanumeric())
                    .last()
                    .unwrap_or(head);

                // Check if reserved word
                let kind = &self.source[head.0..=last.0];
                let kind = Keyword::from_str(kind)
                    .map(TokenKind::Keyword)
                    .unwrap_or_else(|()| TokenKind::Identifier(kind));

                return Some(Ok(Token {
                    span: Span {
                        start: head.0,
                        end: last.0,
                    },
                    kind,
                }));
            }

            // Handle unknown char
            _ => {
                return Some(Err(ScanError::InvalidCharacter { position: head.0 }));
            }
        };

        // All single char tokens are handled down here
        Some(Ok(Token {
            span: Span {
                start: head.0,
                end: head.0,
            },
            kind,
        }))
    }

    /// Ignore whitespace and comments in between token boundaries
    fn trim(&mut self) -> Option<(usize, char)> {
        loop {
            // Trim any whitespace
            self.chars
                .peeking_take_while(|it| it.1.is_whitespace())
                .for_each(drop);

            // Get the head because we only have single peek
            // The ? also handles end of file
            let head = self.chars.next()?;

            // Check for "//" comment starter
            if head.1 == '/' && self.chars.next_if(|it| it.1 == '/').is_some() {
                // Keep consuming chars until a new line
                self.chars
                    .peeking_take_while(|it| it.1 != '\n')
                    .for_each(drop);

                // Try again for potentially more whitespace or comments
                continue;
            }

            // No whitespace, comments, or EOF, assume a valid token is next
            return Some(head);
        }
    }
}
