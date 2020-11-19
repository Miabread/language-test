#[macro_use]
extern crate lalrpop_util;

pub mod codegen;
pub mod parser;
pub mod semantic;

use thiserror::Error;
use CompilerError::*;

pub fn compile(input: &str) -> Result<Vec<u8>, CompilerError> {
    let results = parser::parse(input).map_err(Parse)?;
    let results = semantic::visit_semantic(results)?;
    let results = codegen::codegen(results)?;

    Ok(results)
}

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Error during parsing phase:\n{0}")]
    Parse(String),
    #[error("Error during semantic phase:\n{0}")]
    Semantic(#[from] semantic::SemanticError),
    #[error("Error during codegen phase:\n{0}")]
    Codegen(#[from] codegen::CodegenError),
}
