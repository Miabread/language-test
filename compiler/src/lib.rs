pub mod error;
pub mod token;

pub fn run(source: &str) {
    let (tokens, errors) = token::scan(source);

    if !errors.is_empty() {
        error::report(source, errors);
        return;
    }

    dbg!(tokens);
}
