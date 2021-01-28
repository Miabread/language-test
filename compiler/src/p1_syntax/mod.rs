lalrpop_util::lalrpop_mod!(#[allow(clippy::all)] pub grammar, "/syntax/grammar.rs");

pub fn parse(input: &'_ str) -> Result<File, String> {
    grammar::FileParser::new()
        .parse(input)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Import(ImportItem),
    Func(FuncItem),
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ImportItem {
    pub attributes: Vec<Attribute>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct FuncItem {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_ty: String,
    pub body: Option<Vec<Expression>>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(u8),
    Call {
        leading: Option<Box<Expression>>,
        name: String,
        arguments: Vec<Expression>,
    },
}
