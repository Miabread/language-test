// The goal of this pass is to collect the id of all types
pub mod data;

use super::parser;
pub use data::{File, *};
use std::collections::HashMap;
use thiserror::Error;
use Pass1Error::*;

pub fn pass1(input: parser::File) -> Result<File, Pass1Error> {
    let mut types = HashMap::new();
    let mut functions = Vec::new();

    for item in input.items {
        match item {
            Item::Function(func) => functions.push(FunctionItem::Normal(func)),
            Item::ImportExtern(ImportExtern { items, protocol }) => {
                for item in items {
                    match item {
                        ExternItem::Function(signature) => {
                            functions.push(FunctionItem::Extern(ExternFunction {
                                signature,
                                protocol: protocol.clone(),
                            }))
                        }
                        ExternItem::Type(name) => {
                            if types.contains_key(&name) {
                                return Err(TypeAlreadyExists { name });
                            }

                            if protocol != "builtin" {
                                return Err(RealExternTypeNotSupported { name, protocol });
                            }

                            match &name[..] {
                                "Integer" => {
                                    types.insert(name, Type::Integer);
                                }
                                "Float" => {
                                    types.insert(name, Type::Float);
                                }
                                _ => return Err(UnknownBuiltinType { name }),
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(File { functions, types })
}

#[derive(Debug, Error)]
pub enum Pass1Error {
    #[error("Real external types are not supported (found type {name} with protocol {protocol})")]
    RealExternTypeNotSupported { name: String, protocol: String },
    #[error("Type {name} already exists in scope")]
    TypeAlreadyExists { name: String },
    #[error("Attempted to import unknown builtin type {name}")]
    UnknownBuiltinType { name: String },
}
