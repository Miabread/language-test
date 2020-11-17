#[derive(Debug, Clone)]
pub enum TokenTree {
    Term(Term),
    Identifier(String),
    NumberLiteral(i32),
    Group(Vec<TokenTree>),
    Block(Vec<TokenTree>),
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    FuncKeyword,
    Arrow,
}

impl Term {
    pub fn expect(&self, other: Term) -> Option<String> {
        if *self == other {
            None
        } else {
            Some(format!("expected {:?} token, got {:?}", other, self))
        }
    }
}
