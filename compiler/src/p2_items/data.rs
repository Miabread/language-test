use crate::p1_syntax as syntax;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Protocol {
    sonance_builtin,
    c,
}

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub tys: HashMap<String, TyItem>,
    pub funcs: HashMap<String, FuncItem>,
}

#[derive(Debug, Clone, Default)]
pub struct TyItemRef(pub String);

#[derive(Debug, Clone)]
pub enum TyItem {
    Builtin(BuiltinTy),
}

#[derive(Debug, Clone)]
pub enum BuiltinTy {
    Integer32,
    Float32,
}

#[derive(Debug, Clone)]
pub enum FuncItem {
    Local(LocalFuncItem),
    External(ExternalFuncItem),
}

#[derive(Debug, Clone)]
pub struct LocalFuncItem {
    pub link_name: String,
    pub parameters: Vec<TyItemRef>,
    pub return_ty: TyItemRef,
    pub body: Vec<syntax::Expression>,
}

#[derive(Debug, Clone)]
pub struct ExternalFuncItem {
    pub link_name: String,
    pub protocol: Protocol,
    pub parameters: Vec<TyItemRef>,
    pub return_ty: TyItemRef,
}
