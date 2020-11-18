use logos::Logos;

#[derive(Debug, Clone, PartialEq, Logos)]
pub enum Token {
    #[token("func")]
    FuncKeyword,
    #[token("->")]
    Arrow,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |t| t.slice().to_owned())]
    Identifier(String),
    #[regex("[+-]?[0-9]+", |t| t.slice().parse())]
    NumberLiteral(i32),

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,

    #[error]
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    Error,
}
