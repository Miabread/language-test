use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ParseError<'a> {
    #[error("error while parsing utf8")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("invalid item (found {found})")]
    UnknownItem { found: &'a str },
}
