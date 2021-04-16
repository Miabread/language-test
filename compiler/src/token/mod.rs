mod data;
mod error;
#[cfg(test)]
mod test;

use crate::error::Span;
pub use data::{Keyword, Token, TokenKind};
pub use error::ScanError;
use itertools::{Itertools, MultiPeek, PeekingNext};
use std::str::{CharIndices, FromStr};

pub fn scan(source: &str) -> (Vec<Token<'_>>, Vec<ScanError>) {
    Scanner::new(source).scan()
}

struct Scanner<'src> {
    source: &'src str,
    chars: MultiPeek<CharIndices<'src>>,
}

impl<'src> Scanner<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            chars: source.char_indices().multipeek(),
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
        self.trim();
        let head = self.chars.next()?;

        Some(match head.1 {
            // Single char tokens get handled outside the match
            '(' => Ok(Token::new(TokenKind::OpenParen, Span::new(head.0, head.0))),
            ')' => Ok(Token::new(TokenKind::CloseParen, Span::new(head.0, head.0))),
            '{' => Ok(Token::new(TokenKind::OpenBrace, Span::new(head.0, head.0))),
            '}' => Ok(Token::new(TokenKind::CloseBrace, Span::new(head.0, head.0))),
            ',' => Ok(Token::new(TokenKind::Comma, Span::new(head.0, head.0))),
            ';' => Ok(Token::new(TokenKind::Semicolon, Span::new(head.0, head.0))),

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
                self.source[head.0..=last.0]
                    .parse()
                    .map(TokenKind::Integer)
                    .map(|kind| Token { span, kind })
                    .map_err(|_| ScanError::InvalidInteger { span })
            }

            // Parse string literals
            '"' => {
                // Keep consuming chars until a quote
                self.chars
                    .peeking_take_while(|it| it.1 != '"')
                    .for_each(drop);

                // Expect a closing quote
                let closing = if let Some(closing) = self.next_if_char('"') {
                    closing
                } else {
                    return Some(Err(ScanError::UnterminatedString { start: head.0 }));
                };

                // Slice the leading and trailing quote
                let kind = TokenKind::String(&self.source[head.0 + 1..closing.0]);

                Ok(Token::new(kind, Span::new(head.0, closing.0)))
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

                Ok(Token::new(kind, Span::new(head.0, last.0)))
            }

            // Handle unknown char
            _ => Err(ScanError::InvalidCharacter { position: head.0 }),
        })
    }

    /// Ignore whitespace and comments in between token boundaries
    fn trim(&mut self) {
        // Note the condition is in the middle of the loop
        while {
            // Trim any whitespace
            self.chars
                .peeking_take_while(|it| it.1.is_whitespace())
                .for_each(drop);

            // Check for "//" comment starter
            self.peek_if_char('/').is_some() && self.next_if_char('/').is_some()
        } {
            // Keep consuming chars until a new line
            self.chars
                .peeking_take_while(|it| it.1 != '\n')
                .for_each(drop);
        }
    }

    fn peek_if_char(&mut self, char: char) -> Option<&(usize, char)> {
        self.chars.peek().filter(|it| it.1 == char)
    }

    fn next_if_char(&mut self, char: char) -> Option<(usize, char)> {
        self.chars.peeking_next(|it| it.1 == char)
    }
}
