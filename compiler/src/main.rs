use clap::Clap;
use std::{
    env, fs,
    io::{self, Write},
};

#[derive(Debug, Clone, Clap)]
#[clap(version = "0.1.0", author = "jamesBeeProg <jamesBeeProg@gmail.com>")]
struct Settings {
    input: Option<String>,
}

fn main() {
    let settings = Settings::parse();

    if let Some(input) = settings.input {
        run_file(input);
    } else {
        run_repl();
    }
}

fn run_file(input: String) {
    let cwd = env::current_dir().expect("couldn't get current dir");
    let input = fs::read_to_string(cwd.join(input)).expect("couldn't read source file");
    println!();
    compiler::run(&input);
}

fn run_repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "//exit" {
            break;
        }

        println!();
        compiler::run(input);
        println!();
    }
}
