use logos::Logos; 

#[derive(Debug, Clone, PartialEq, Logos)]
pub enum Token {
    #[token("func")]
    FuncKeyword,
    #[token("->")]
    Arrow,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex("[+-]?[0-9]+")]
    NumberLiteral,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBracket,
    #[token("}")]
    CloseBracket,

    #[error]
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    Error,
}
