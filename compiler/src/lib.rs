pub mod error;
pub mod parser;
pub mod token;

pub fn run(source: &str) {
    let reporter = error::Reporter::new(source);

    for token in token::scan(source) {
        match token {
            Ok(token) => println!("{:?}", token),
            Err(err) => reporter.report(&[err.into()]),
        }
    }

    // let parsed = parser::parse(tokens.into_iter());

    // match parsed {
    //     Ok(parsed) => println!("{:?}", parsed),
    //     Err(errors) => {
    //         let errors: Vec<_> = errors.into_iter().map(Into::into).collect();
    //         error::Reporter::new(source).report(&errors);
    //     }
    // }
}
