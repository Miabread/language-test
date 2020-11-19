lalrpop_mod!(#[allow(clippy::all)] pub grammar, "/parser/grammar.rs");

pub fn parse(input: &str) -> Result<Function, String> {
    grammar::FunctionParser::new()
        .parse(input)
        .map_err(|e| e.to_string())
}

pub struct Function {
    pub name: String,
    pub ty: String,
    pub body: i32,
}
