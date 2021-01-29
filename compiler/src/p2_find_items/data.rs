use {crate::p1_syntax as syntax, std::collections::HashMap};

#[derive(Debug, Clone)]
pub struct Program {
    pub tys: HashMap<String, RawTyPrototype>,
    pub funcs: HashMap<String, RawFuncPrototype>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ExternalProtocol {
    sonance_builtin,
    c,
}

#[derive(Debug, Clone)]
pub struct RawTyPrototype {
    pub linkage: String,
}

#[derive(Debug, Clone)]
pub struct RawFuncPrototype {
    pub linkage: RawFuncLinkage,
    pub parameters: Vec<String>,
    pub return_ty: String,
}

#[derive(Debug, Clone)]
pub enum RawFuncLinkage {
    Local(Vec<syntax::Expression>),
    Foreign(String),
    Builtin(String),
}
