use tree_sitter::{Language, Parser};

pub use tree_sitter::Tree;

extern "C" {
    fn tree_sitter_sonance() -> Language;
}

#[derive(Debug, Clone)]
pub struct File<'a> {
    pub name: &'a str,
    pub number: u8,
}

pub fn parse(source: &[u8]) -> File {
    let mut parser = Parser::new();
    parser
        .set_language(unsafe { tree_sitter_sonance() })
        .unwrap();
    let tree = parser.parse(source, None).unwrap();

    File {
        name: tree
            .root_node()
            .named_child(0)
            .unwrap()
            .utf8_text(source)
            .unwrap(),
        number: tree
            .root_node()
            .named_child(1)
            .unwrap()
            .utf8_text(source)
            .unwrap()
            .parse()
            .unwrap(),
    }
}
