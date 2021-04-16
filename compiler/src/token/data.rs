use crate::error::Span;
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::FromStr,
};

#[derive(Clone, PartialEq)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub span: Span,
}

impl<'src> Token<'src> {
    pub fn new(kind: TokenKind<'src>, span: Span) -> Self {
        Self { kind, span }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} [{}]", self.kind, self.span)
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "token({:?}, {})", self.kind, self.span)
    }
}

// Note: variant order should be the same as match order in scan()
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind<'src> {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Semicolon,
    Integer(i64),
    String(&'src str),
    Keyword(Keyword),
    Identifier(&'src str),
}

impl Display for TokenKind<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                TokenKind::OpenParen => "(",
                TokenKind::CloseParen => ")",
                TokenKind::OpenBrace => "{",
                TokenKind::CloseBrace => "}",
                TokenKind::Comma => ",",
                TokenKind::Semicolon => ";",
                TokenKind::Integer(it) => return write!(f, "{}", it),
                TokenKind::String(it) => return write!(f, r#""{}""#, it),
                TokenKind::Keyword(it) => return write!(f, "{}", it),
                TokenKind::Identifier(it) => return write!(f, "{}", it),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Func,
    Debug,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Keyword::Func => "func",
                Keyword::Debug => "debug",
            }
        )
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "func" => Keyword::Func,
            "debug" => Keyword::Debug,
            _ => return Err(()),
        })
    }
}
