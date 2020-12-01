pub use super::parser::data::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct File {
    pub types: HashMap<String, Type>,
    pub functions: Vec<FunctionItem>,
}

#[derive(Debug, Clone)]
pub enum FunctionItem {
    Normal(Function),
    Extern(ExternFunction),
}

#[derive(Debug, Clone)]
pub struct ExternFunction {
    pub signature: FunctionSignature,
    pub protocol: String,
}

#[derive(Debug, Clone)]
pub enum Type {
    Integer,
    Float,
}
