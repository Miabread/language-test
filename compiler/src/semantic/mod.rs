pub mod symbol_table;

use crate::parser;
use thiserror::Error;
use SemanticError::*;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub return_ty: String,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub signature: FunctionSignature,
    pub body: i32,
}

pub fn semanticize(input: parser::Function) -> Result<Function, SemanticError> {
    if input.return_ty != "I32" {
        return Err(JustI32);
    }

    Ok(Function {
        signature: FunctionSignature {
            name: input.name,
            return_ty: input.return_ty,
        },
        body: input.body,
    })
}

#[derive(Debug, Clone, Error)]
pub enum SemanticError {
    #[error("Only the I32 type has been implemented")]
    JustI32,
}
