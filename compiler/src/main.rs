use {
    clap::Clap,
    compiler::compile,
    std::{env, fs, io::Write},
    syntax::parse,
};

#[derive(Debug, Clone, Clap)]
#[clap(version = "0.1.0", author = "jamesBeeProg <jamesBeeProg@gmail.com>")]
struct Settings {
    #[clap(default_value = "indev/input.son")]
    input: String,
    #[clap(default_value = "indev/output.o")]
    output: String,
}

fn main() {
    let cwd = env::current_dir().expect("couldn't get current dir");
    let settings = Settings::parse();

    let input = fs::read(cwd.join(settings.input)).expect("couldn't read source file");
    let input = parse(&input);

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
