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
    OpenAngle,
    CloseAngle,
    Comma,
    Semicolon,
    DotSymbol,
    AtSymbol,
    Integer(i64),
    Character(char),
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
                Self::OpenParen => "(",
                Self::CloseParen => ")",
                Self::OpenBrace => "{",
                Self::CloseBrace => "}",
                Self::OpenAngle => "<",
                Self::CloseAngle => ">",
                Self::Comma => ",",
                Self::Semicolon => ";",
                Self::DotSymbol => ".",
                Self::AtSymbol => "@",
                Self::Integer(it) => return write!(f, "{}", it),
                Self::Character(it) => return write!(f, "'{}'", it),
                Self::String(it) => return write!(f, r#""{}""#, it),
                Self::Keyword(it) => return write!(f, "{}", it),
                Self::Identifier(it) => return write!(f, "{}", it),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Struct,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Struct => "struct",
            }
        )
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "struct" => Self::Struct,
            _ => return Err(()),
        })
    }
}
