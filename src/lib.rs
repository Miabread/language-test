mod codegen;
mod lexer;
mod parser;

pub fn compile(input: &str) {
    let results = lexer::lex(input);

    if !results.errors.is_empty() {
        println!("Error durring lexing phase:");
        for error in results.errors {
            println!("{}", error);
        }
    }

    let results = match parser::parse(results.output) {
        Err(error) => {
            println!("Error during parse phase: {}", error);
            return;
        }
        Ok(results) => results,
    };

    let results = match codegen::codegen(results) {
        Err(error) => {
            println!("Error during codegen phase: {}", error);
            return;
        }
        Ok(results) => results,
    };

    println!("{:?}", results);
}
