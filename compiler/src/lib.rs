mod codegen;
mod lexer;
mod parser;

use thiserror::Error;

pub fn compile(input: &str) -> Result<Vec<u8>, CompilerError> {
    let results = lexer::lex(input);

    if !results.errors.is_empty() {
        return Err(CompilerError::Lex(results.errors));
    }

    let results = parser::parse(results.output)?;
    let results = codegen::codegen(results)?;

    Ok(results)
}

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Error during lexing phase")]
    Lex(Vec<lexer::LexerError>),
    #[error("Error during parsing phase")]
    Parse(#[from] parser::ParserError),
    #[error("Error during codegen phase")]
    Codegen(#[from] codegen::CodegenError),
}
