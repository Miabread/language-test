pub mod token;

pub fn run(source: &str) {
    let tokens = token::scan(source);
    let _ = dbg!(tokens);
}
