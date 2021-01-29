pub mod p1_syntax;
pub mod p2_find_items;

use thiserror::Error;

pub fn compile(input: &str) -> Result<p2_find_items::Program, CompileError> {
    let output = p1_syntax::parse(input).map_err(CompileError::SyntaxError)?;
    let output = p2_find_items::find_items(output)?;
    Ok(output)
}

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Phase 1: Syntax error {0}")]
    SyntaxError(String),
    #[error("Phase 2: find_items error")]
    FindItemsError(#[from] p2_find_items::FindItemsError),
}
