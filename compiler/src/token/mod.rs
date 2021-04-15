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

        while let Some(head) = self.chars.next() {
            let kind = match head.1 {
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
                '/' if self.chars.next_if(|it| it.1 == '/').is_some() => {
                    // Keep consuming chars until a new line
                    self.chars
                        .peeking_take_while(|it| it.1 != '\n')
                        .for_each(drop);
                    continue;
                }

                // Parse number literals
                number if number.is_ascii_digit() => {
                    // Keep consuming digit chars, last digit needed for slice
                    let last = self
                        .chars
                        .peeking_take_while(|it| it.1.is_ascii_digit())
                        .last()
                        .unwrap_or(head);

                    // TODO: Implement float literals. Delayed due to needing double look ahead.

                    // Parse the digits into a number
                    let kind = TokenKind::Integer(
                        self.source[head.0..=last.0]
                            .parse()
                            .expect("parse to work since we only pass digits"),
                    );

                    tokens.push(Token {
                        span: Span {
                            start: head.0,
                            end: last.0,
                        },
                        kind,
                    });
                    continue;
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
                        errors.push(ScanError::UnterminatedString { start: head.0 });
                        break;
                    };

                    // Slice the leading and trailing quote
                    let kind = TokenKind::String(&self.source[head.0 + 1..closing.0]);

                    tokens.push(Token {
                        span: Span {
                            start: head.0,
                            end: closing.0,
                        },
                        kind,
                    });
                    continue;
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

                    tokens.push(Token {
                        span: Span {
                            start: head.0,
                            end: last.0,
                        },
                        kind,
                    });
                    continue;
                }

                // Handle unknown char
                _ => {
                    errors.push(ScanError::InvalidCharacter { position: head.0 });
                    continue;
                }
            };

            // All single char tokens are handled down here
            tokens.push(Token {
                span: Span {
                    start: head.0,
                    end: head.0,
                },
                kind,
            })
        }

        (tokens, errors)
    }
}
