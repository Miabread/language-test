// The goal of this pass is to collect the id of all types
pub mod data;

use super::parser;
pub use data::{File, *};
use thiserror::Error;
use Pass1Error::*;

impl parser::File {
    fn pass1(self) -> Result<File, Pass1Error> {
        let mut context = File::default();

        for item in self.items {
            item.pass1(&mut context)?;
        }

        Ok(context)
    }
}

impl Item {
    fn pass1(self, context: &mut File) -> Result<(), Pass1Error> {
        match self {
            Self::Function(func) => context.functions.push(FunctionItem::Normal(func)),
            Self::ImportExtern(import) => {
                for item in import.items {
                    item.pass1(&import.protocol, context)?;
                }
            }
        }

        Ok(())
    }
}

impl ExternItem {
    fn pass1(self, protocol: &str, context: &mut File) -> Result<(), Pass1Error> {
        match self {
            Self::Function(signature) => {
                context.functions.push(FunctionItem::Extern(ExternFunction {
                    signature,
                    protocol: protocol.to_owned(),
                }))
            }

            Self::Type(name) => {
                if context.types.contains_key(&name) {
                    return Err(TypeAlreadyExists { name });
                }

                if protocol != "builtin" {
                    return Err(RealExternTypeNotSupported {
                        name,
                        protocol: protocol.to_owned(),
                    });
                }

                match &name[..] {
                    "Integer" => {
                        context.types.insert(name, Type::Integer);
                    }
                    "Float" => {
                        context.types.insert(name, Type::Float);
                    }
                    _ => return Err(UnknownBuiltinType { name }),
                }
            }
        }

        Ok(())
    }
}

pub fn pass1(input: parser::File) -> Result<File, Pass1Error> {
    input.pass1()
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
