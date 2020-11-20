use crate::parser;
use thiserror::Error;
use SemanticError::*;

#[derive(Debug, Clone)]
pub struct PartialFile {
    pub items: Vec<PartialItem>,
}

impl parser::File {
    pub fn visit_semantic(self) -> Result<PartialFile, SemanticError> {
        Ok(PartialFile {
            items: self
                .items
                .into_iter()
                .map(|it| it.visit_semantic())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum PartialItem {
    Function(PartialFunction),
    Import(Import),
}

impl parser::Item {
    pub fn visit_semantic(self) -> Result<PartialItem, SemanticError> {
        Ok(match self {
            Self::Function(func) => PartialItem::Function(func.visit_semantic()?),
            Self::Import(import) => PartialItem::Import(import.visit_semantic()?),
        })
    }
}

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

#[derive(Debug, Clone)]
pub struct Import {
    pub signatures: Vec<FunctionSignature>,
}

impl parser::Import {
    pub fn visit_semantic(self) -> Result<Import, SemanticError> {
        Ok(Import {
            signatures: self
                .signatures
                .into_iter()
                .map(|it| it.visit_semantic())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Clone)]
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
pub struct File {
    pub items: Vec<Item>,
}

impl PartialFile {
    pub fn visit_semantic(self) -> Result<File, SemanticError> {
        Ok(File {
            items: self
                .items
                .into_iter()
                .map(|it| it.visit_semantic())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    Import(Import),
}

impl PartialItem {
    pub fn visit_semantic(self) -> Result<Item, SemanticError> {
        Ok(match self {
            Self::Function(func) => Item::Function(func.visit_semantic()?),
            Self::Import(import) => Item::Import(import),
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

pub fn visit_semantic(input: parser::File) -> Result<File, SemanticError> {
    input.visit_semantic()?.visit_semantic()
}

#[derive(Debug, Clone, Error)]
pub enum SemanticError {
    #[error("Only the I32 type has been implemented")]
    JustI32,
}
