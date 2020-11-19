lalrpop_mod!(#[allow(clippy::all)] pub grammar, "/parser/grammar.rs");

pub fn parse(input: &str) -> Result<File, String> {
    grammar::FileParser::new()
        .parse(input)
        .map_err(|e| format!("{:?}", e))
}

#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
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
    Literal(i32),
}
