pub mod data;
pub mod error;
pub mod walk;

pub use error::ParseError;

pub type Result<'a, T, E = ParseError<'a>> = std::result::Result<T, E>;

use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_sonance() -> Language;
}

pub fn parse(source: &[u8]) -> Result<data::File> {
    let mut parser = Parser::new();
    parser
        .set_language(unsafe { tree_sitter_sonance() })
        .unwrap();
    let tree = parser.parse(source, None).unwrap();
    walk::file(tree.walk(), source)
}
