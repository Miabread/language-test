pub mod p1_syntax;
pub mod p2_items;
pub mod p3_body;
pub mod p4_codegen;

use thiserror::Error;

pub fn compile(input: &str) -> Result<Vec<u8>, CompileError> {
    let output = p1_syntax::parse(input).map_err(CompileError::Syntax)?;
    let output = p2_items::check_items(output)?;
    let output = p3_body::check_bodies(output)?;
    let output = p4_codegen::compile(output)?;
    Ok(output)
}

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Phase 1: Syntax error {0}")]
    Syntax(String),
    #[error("Phase 2: Item error")]
    Items(#[from] p2_items::ItemsError),
    #[error("Phase 3: Body error")]
    Body(#[from] p3_body::BodyError),
    #[error("Phase 4: Codegen error")]
    Codegen(#[from] p4_codegen::CodegenError),
}
