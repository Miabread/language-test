use crate::p2_items as items;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Program {
    pub items: items::Program,
    pub func_bodies: Vec<FuncBody>,
}

#[derive(Debug, Clone, Default)]
pub struct FuncItemRef(String);

impl FuncItemRef {
    pub fn new(name: String, funcs: &HashMap<String, items::FuncItem>) -> Option<Self> {
        if funcs.contains_key(&name) {
            Some(FuncItemRef(name))
        } else {
            None
        }
    }

    pub fn get<'a>(&'a self, funcs: &'a HashMap<String, items::FuncItem>) -> &'a items::FuncItem {
        &funcs[&self.0]
    }
}

#[derive(Debug, Clone)]
pub struct FuncBody {
    pub signature: FuncItemRef,
    pub body: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(u8),
    Call {
        leading: Option<Box<Expression>>,
        name: FuncItemRef,
        arguments: Vec<Expression>,
    },
}
