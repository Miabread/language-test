#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    ImportExtern(ImportExtern),
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub return_ty: String,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub signature: FunctionSignature,
    pub body: Expression,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i32),
    Float(f32),
}

#[derive(Debug, Clone)]
pub struct ImportExtern {
    pub protocol: String,
    pub items: Vec<ExternItem>,
}

#[derive(Debug, Clone)]
pub enum ExternItem {
    Function(FunctionSignature),
    Type(String),
}
