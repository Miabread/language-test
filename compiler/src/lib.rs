#[macro_use]
extern crate lalrpop_util;

pub mod parser;
pub mod pass1;

use thiserror::Error;

pub fn compile(input: &str) -> Result<pass1::File, CompilerError> {
    let results = parser::parse(input)?;
    let results = pass1::pass1(results)?;

    Ok(results)
}

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Failed to parse file")]
    Parse(#[from] parser::ParseError),
    #[error("Checks failed during pass1 phase")]
    Pass1(#[from] pass1::Pass1Error),
}
