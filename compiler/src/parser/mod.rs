lalrpop_mod!(#[allow(clippy::all)] pub grammar, "/parser/grammar.rs");

pub fn parse(input: &str) -> Result<Function, String> {
    grammar::FunctionParser::new()
        .parse(input)
        .map_err(|e| format!("{:?}", e))
}

pub struct Function {
    pub name: String,
    pub return_ty: String,
    pub body: i32,
}
