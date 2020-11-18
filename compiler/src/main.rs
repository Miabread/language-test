use clap::Clap;
use language_test_compiler::compile;
use std::{env, fs, io::Write};

#[derive(Debug, Clone, Clap)]
#[clap(version = "0.1.0", author = "jamesBeeProg <jamesBeeProg@gmail.com>")]
struct Settings {
    input: String,
    output: String,
}

fn main() {
    let cwd = env::current_dir().expect("couldn't get current dir");
    let settings = Settings::parse();

    let input = fs::read_to_string(cwd.join(settings.input)).expect("couldn't read source file");

    let compiled = match compile(&input) {
        Ok(compiled) => compiled,
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    };

    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(cwd.join(settings.output))
        .unwrap()
        .write_all(&compiled)
        .unwrap();
}
