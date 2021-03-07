use clap::Clap;
use std::{
    env, fs,
    io::{self, Write},
};

#[derive(Debug, Clone, Clap)]
#[clap(version = "0.1.0", author = "jamesBeeProg <jamesBeeProg@gmail.com>")]
struct Settings {
    #[clap(subcommand)]
    sub: SubCommand,
}

#[derive(Debug, Clone, Clap)]
enum SubCommand {
    File {
        input: String,
    },
    Repl,
    ErrorTest {
        input: String,
        start: usize,
        end: usize,
    },
}

fn main() {
    let settings = Settings::parse();

    match settings.sub {
        SubCommand::File { input } => run_file(input),
        SubCommand::Repl => run_repl(),
        SubCommand::ErrorTest { input, start, end } => run_error_test(input, start, end),
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

fn run_error_test(input: String, start: usize, end: usize) {
    let cwd = env::current_dir().expect("couldn't get current dir");
    let input = fs::read_to_string(cwd.join(input)).expect("couldn't read source file");

    use compiler::error::*;

    Reporter::new(&input)
        .report(&[Citation::error("Test error".to_owned()).span(Span { start, end }, None)]);
}
