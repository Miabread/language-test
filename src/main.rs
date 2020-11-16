use clap::Clap;
use std::{error, env, fs};
use language_test::Token;
use logos::Logos;

#[derive(Debug, Clone, Clap)]
#[clap(version = "0.1.0", author = "jamesBeeProg <jamesBeeProg@gmail.com>")]
struct Settings {
    input: String,
}

type Result<T, E = Box<dyn error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let settings = Settings::parse();

    let input = env::current_dir()?.join(settings.input);
    let input = fs::read_to_string(input)?;

    println!("{:#?}", Token::lexer(&input).collect::<Vec<_>>());

    Ok(())
}
