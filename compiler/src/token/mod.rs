mod data;
mod error;
#[cfg(test)]
mod test;

use crate::error::Span;
pub use data::{Keyword, Token, TokenKind};
pub use error::ScanError;
use itertools::{Itertools, MultiPeek, PeekingNext};
use std::str::{CharIndices, FromStr};

type ScanResult<'src> = Result<Token<'src>, ScanError>;
type Head = (usize, char);

pub fn scan(source: &str) -> (Vec<Token<'_>>, Vec<ScanError>) {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for eee in Scanner::new(source) {
        match eee {
            Ok(token) => tokens.push(token),
            Err(error) => errors.push(error),
        }
    }

    (tokens, errors)
}

struct Scanner<'src> {
    source: &'src str,
    chars: MultiPeek<CharIndices<'src>>,
}

impl<'src> Iterator for Scanner<'src> {
    type Item = ScanResult<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim();
        let head = self.chars.next()?;

        self.simple_tokens(head)
            .or_else(|| self.number_literals(head))
            .or_else(|| self.string_literal(head))
            .or_else(|| self.identifier_or_keyword(head))
            .or(Some(Err(ScanError::InvalidCharacter { position: head.0 })))
    }
}

impl<'src> Scanner<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            chars: source.char_indices().multipeek(),
        }
    }

    fn simple_tokens(&mut self, head: Head) -> Option<ScanResult<'src>> {
        let kind = match head.1 {
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            _ => return None,
        };

        Some(Ok(Token::new(kind, Span::new(head.0, head.0))))
    }

    fn number_literals(&mut self, head: Head) -> Option<ScanResult<'src>> {
        if !head.1.is_ascii_digit() {
            return None;
        }

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
        Some(
            self.source[head.0..=last.0]
                .parse()
                .map(TokenKind::Integer)
                .map(|kind| Token { span, kind })
                .map_err(|_| ScanError::InvalidInteger { span }),
        )
    }

    fn string_literal(&mut self, head: Head) -> Option<ScanResult<'src>> {
        if head.1 != '"' {
            return None;
        }

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

        Some(Ok(Token::new(kind, Span::new(head.0, closing.0))))
    }

    fn identifier_or_keyword(&mut self, head: Head) -> Option<ScanResult<'src>> {
        if !head.1.is_alphabetic() {
            return None;
        }

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

        Some(Ok(Token::new(kind, Span::new(head.0, last.0))))
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

    fn peek_if_char(&mut self, char: char) -> Option<&Head> {
        self.chars.peek().filter(|it| it.1 == char)
    }

    fn next_if_char(&mut self, char: char) -> Option<Head> {
        self.chars.peeking_next(|it| it.1 == char)
    }
}
