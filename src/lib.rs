mod token_tree;

pub fn compile(input: &str) -> token_tree::CompileResult {
    token_tree::compile(input)
}
