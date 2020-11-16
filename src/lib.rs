use logos::{Lexer, Logos};

#[derive(Debug, Clone, PartialEq, Logos)]
enum Token {
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

#[derive(Debug, Clone)]
pub enum TokenTree {
    FuncKeyword,
    Arrow,
    Identifier(String),
    NumberLiteral(i32),
    Group(Vec<TokenTree>),
    Block(Vec<TokenTree>),
    Error,
}

#[derive(Debug, Clone, Copy)]
enum Lookout {
    None,
    Paren,
    Brace,
}

#[derive(Debug, Clone)]
pub struct CompileResult {
    pub output: Vec<TokenTree>,
    pub errors: Vec<&'static str>,
}

pub fn compile(input: &str) -> CompileResult {
    let mut input = Token::lexer(&input);
    convert(&mut input, Lookout::None)
}

fn convert(input: &mut Lexer<Token>, lookout: Lookout) -> CompileResult {
    let mut errors = Vec::new();
    let mut output = Vec::new();

    while let Some(token) = input.next() {
        output.push(match token {
            Token::FuncKeyword => TokenTree::FuncKeyword,
            Token::Arrow => TokenTree::Arrow,
            Token::Identifier(ident) => TokenTree::Identifier(ident),
            Token::NumberLiteral(num) => TokenTree::NumberLiteral(num),
            Token::Error => TokenTree::Error,

            Token::OpenParen => {
                let result = convert(input, Lookout::Paren);
                errors.extend(result.errors);

                TokenTree::Group(result.output)
            }
            Token::CloseParen => {
                if let Lookout::Paren = lookout {
                    break;
                } else {
                    errors.push("unmatched paren");
                    continue;
                }
            }

            Token::OpenBrace => {
                let result = convert(input, Lookout::Brace);
                errors.extend(result.errors);

                TokenTree::Block(result.output)
            }
            Token::CloseBrace => {
                if let Lookout::Brace = lookout {
                    break;
                } else {
                    errors.push("unmatched brace");
                    continue;
                }
            }
        })
    }

    CompileResult { output, errors }
}
