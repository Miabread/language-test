pub mod error;
pub mod token;

pub fn run(source: &str) {
    let (tokens, errors) = token::scan(source);

    if !errors.is_empty() {
        let errors: Vec<_> = errors.into_iter().map(Into::into).collect();
        error::Reporter::new(source).report(&errors);
        return;
    }

    dbg!(tokens);
}
