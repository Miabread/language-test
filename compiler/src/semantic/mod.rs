pub mod symbol_table;

use crate::parser;
use thiserror::Error;
use SemanticError::*;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub return_ty: String,
}

impl parser::FunctionSignature {
    pub fn visit_semantic(self) -> Result<FunctionSignature, SemanticError> {
        if self.return_ty != "I32" {
            return Err(JustI32);
        }

        Ok(FunctionSignature {
            name: self.name,
            return_ty: self.return_ty,
        })
    }
}

pub struct PartialFunction {
    pub signature: FunctionSignature,
    pub partial_body: parser::Expression,
}

impl parser::Function {
    pub fn visit_semantic(self) -> Result<PartialFunction, SemanticError> {
        Ok(PartialFunction {
            signature: self.signature.visit_semantic()?,
            partial_body: self.body,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub signature: FunctionSignature,
    pub body: Expression,
}

impl PartialFunction {
    pub fn visit_semantic(self) -> Result<Function, SemanticError> {
        Ok(Function {
            signature: self.signature,
            body: self.partial_body.visit_semantic()?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(i32),
}

impl parser::Expression {
    pub fn visit_semantic(self) -> Result<Expression, SemanticError> {
        Ok(match self {
            Self::Literal(num) => Expression::Literal(num),
        })
    }
}

pub fn visit_semantic(input: parser::Function) -> Result<Function, SemanticError> {
    input.visit_semantic()?.visit_semantic()
}

#[derive(Debug, Clone, Error)]
pub enum SemanticError {
    #[error("Only the I32 type has been implemented")]
    JustI32,
}
