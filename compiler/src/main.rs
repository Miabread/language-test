use clap::Clap;
use language_test_compiler::compile;
use std::{env, fs};

#[derive(Debug, Clone, Clap)]
#[clap(version = "0.1.0", author = "jamesBeeProg <jamesBeeProg@gmail.com>")]
struct Settings {
    input: String,
    output: String,
}

fn main() -> anyhow::Result<()> {
    let cwd = env::current_dir()?;
    let settings = Settings::parse();

    let input = fs::read_to_string(cwd.join(settings.input))?;

    let compiled = compile(&input)?;

    println!("{:#?}", compiled);

    // fs::OpenOptions::new()
    //     .create(true)
    //     .write(true)
    //     .truncate(true)
    //     .open(cwd.join(settings.output))
    //     .unwrap()
    //     .write_all(&compiled)
    //     .unwrap();

    Ok(())
}
