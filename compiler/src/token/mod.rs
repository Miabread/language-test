mod data;
mod error;
#[cfg(test)]
mod test;

use std::str::FromStr;

use crate::error::Span;
pub use data::{Keyword, Token, TokenKind};
pub use error::ScanError;
use itertools::Itertools;

pub fn scan(source: &str) -> (Vec<Token<'_>>, Vec<ScanError>) {
    let mut chars = source.char_indices().peekable();
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    'outer: while let Some(char) = chars.next() {
        let kind = match char.1 {
            // Ignore whitespace in between token boundaries
            char if char.is_whitespace() => continue,

            // Single char tokens get handled outside the match
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,

            // Check for "//" comment starter
            '/' if chars.next_if(|it| it.1 == '/').is_some() => {
                // Keep consuming chars until a new line
                chars.peeking_take_while(|it| it.1 != '\n').for_each(drop);
                continue;
            }

            // Parse number literals
            number if number.is_ascii_digit() => {
                // Last digit needed for slice
                let mut last = char;

                // Keep consuming digit chars
                while let Some(char) = chars.peek() {
                    if char.1.is_ascii_digit() {
                        last = chars.next().expect("due to peek above");
                    } else {
                        break;
                    }
                }

                // TODO: Implement float literals. Delayed due to needing double look ahead.

                // Parse the digits into a number
                let kind = TokenKind::Integer(
                    source[char.0..=last.0]
                        .parse()
                        .expect("parse to work since we only pass digits"),
                );

                tokens.push(Token {
                    span: Span {
                        start: char.0,
                        end: last.0,
                    },
                    kind,
                });
                continue;
            }

            // Parse string literals
            '"' => {
                // Keep consuming chars until a quote
                let closing = loop {
                    match chars.next() {
                        Some(closing @ (_, '"')) => break closing,
                        None => {
                            errors.push(ScanError::UnterminatedString { start: char.0 });
                            break 'outer;
                        }
                        _ => {}
                    }
                };

                // Slice the leading and trailing quote
                let kind = TokenKind::String(&source[char.0 + 1..closing.0]);

                tokens.push(Token {
                    span: Span {
                        start: char.0,
                        end: closing.0,
                    },
                    kind,
                });
                continue;
            }

            letter if letter.is_alphabetic() => {
                // Last char needed for slice
                let mut last = char;

                // Keep consuming chars
                while let Some(char) = chars.peek() {
                    if char.1.is_alphanumeric() {
                        last = chars.next().expect("due to peek above");
                    } else {
                        break;
                    }
                }

                // Check if reserved word
                let kind = &source[char.0..=last.0];
                let kind = Keyword::from_str(kind)
                    .map(TokenKind::Keyword)
                    .unwrap_or_else(|()| TokenKind::Identifier(kind));

                tokens.push(Token {
                    span: Span {
                        start: char.0,
                        end: last.0,
                    },
                    kind,
                });
                continue;
            }

            // Handle unknown char
            _ => {
                errors.push(ScanError::InvalidCharacter { position: char.0 });
                continue;
            }
        };

        // All single char tokens are handled down here
        tokens.push(Token {
            span: Span {
                start: char.0,
                end: char.0,
            },
            kind,
        })
    }

    (tokens, errors)
}
