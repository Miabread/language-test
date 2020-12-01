#[macro_use]
extern crate lalrpop_util;

pub mod parser;

use thiserror::Error;
use CompilerError::*;

pub fn compile(input: &str) -> Result<parser::File, CompilerError> {
    let results = parser::parse(input).map_err(Parse)?;

    Ok(results)
}

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Error during parsing phase:\n{0}")]
    Parse(String),
}
