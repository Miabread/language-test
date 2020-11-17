mod lexer;

pub fn compile(input: &str) -> lexer::CompileResult {
    lexer::compile(input)
}
