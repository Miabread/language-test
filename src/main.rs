use clap::Clap;
use language_test::compile;
use std::{env, fs};

#[derive(Debug, Clone, Clap)]
#[clap(version = "0.1.0", author = "jamesBeeProg <jamesBeeProg@gmail.com>")]
struct Settings {
    input: String,
}

fn main() {
    let settings = Settings::parse();

    let input = env::current_dir()
        .expect("couldn't get current dir")
        .join(settings.input);
    let input = fs::read_to_string(input).expect("couldn't read source file");

    println!("{:#?}", compile(&input));
}
