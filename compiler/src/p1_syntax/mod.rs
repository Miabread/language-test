lalrpop_util::lalrpop_mod!(#[allow(clippy::all)] pub grammar, "/p1_syntax/grammar.rs");

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
    External(ExternalItem),
    Func(FuncItem),
}

#[derive(Debug, Clone)]
pub struct ExternalItem {
    pub protocol: String,
    pub items: Vec<FuncSignature>,
}

#[derive(Debug, Clone)]
pub struct FuncSignature {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_ty: String,
}

#[derive(Debug, Clone)]
pub struct FuncItem {
    pub signature: FuncSignature,
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
