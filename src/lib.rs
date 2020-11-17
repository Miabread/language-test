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

    let results = parser::parse(results.output);

    match results {
        Ok((name, number)) => println!("name: {}, number: {}", name, number),
        Err(error) => println!("Error during parse phase: {}", error),
    }
}
