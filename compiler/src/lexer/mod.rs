pub mod token;
pub mod token_tree;

use logos::{Lexer, Logos};
use token::Token;
use token_tree::TokenTree;

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

pub fn lex(input: &str) -> CompileResult {
    let mut input = Token::lexer(&input);
    convert(&mut input, Lookout::None)
}

fn convert(input: &mut Lexer<Token>, lookout: Lookout) -> CompileResult {
    let mut output = Vec::new();
    let mut errors = Vec::new();

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
