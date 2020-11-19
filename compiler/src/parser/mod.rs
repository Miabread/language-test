lalrpop_mod!(#[allow(clippy::all)] pub grammar, "/parser/grammar.rs");

pub fn parse(input: &str) -> Result<Function, String> {
    grammar::FunctionParser::new()
        .parse(input)
        .map_err(|e| format!("{:?}", e))
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
