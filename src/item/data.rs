use crate::ast::data::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub types: Vec<EnumItem>,
    pub funcs: Vec<FuncItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumItem {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncItem {
    pub name: String,
    pub body: Expr,
    pub body_ty: String,
}
