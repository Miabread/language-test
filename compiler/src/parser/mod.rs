lalrpop_mod!(#[allow(clippy::all)] pub grammar, "/parser/grammar.rs");

pub mod data;

pub use data::*;
use std::fmt;
use thiserror::Error;

pub fn parse(input: &str) -> Result<File, ParseError> {
    grammar::FileParser::new()
        .parse(input)
        .map_err(|e| ParseError(format!("{:?}", e)))
}

#[derive(Debug, Clone, Error)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
