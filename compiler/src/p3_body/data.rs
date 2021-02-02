use crate::p2_items as items;

#[derive(Debug, Clone)]
pub struct Program {
    pub items: items::Program,
    pub func_bodies: Vec<FuncBody>,
}

#[derive(Debug, Clone, Default)]
pub struct FuncItemRef(pub String);

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
