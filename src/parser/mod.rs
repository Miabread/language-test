use crate::lexer::token_tree::{TokenTree, TokenTreeType};
use thiserror::Error;
use ParserError::*;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("invalid token (expected {expected:?}, found {found:?})")]
    InvalidToken {
        expected: TokenTreeType,
        found: TokenTree,
    },
    #[error("unexpected end of file")]
    InvalidEndOfFile,
    #[error("unexpected trailing token: {found:?}")]
    InvalidTrailingTokens { found: TokenTree },
    #[error("all types must be I32 for now")]
    TypeError,
}

macro_rules! next_token {
    ($input: ident, $func: expr) => {
        $input.next().ok_or(InvalidEndOfFile).and_then($func);
    };
}

macro_rules! end_tokens {
    ($input: ident) => {
        if let Some(found) = $input.next() {
            return Err(InvalidTrailingTokens { found });
        }
    };
}

pub fn parse(input: Vec<TokenTree>) -> Result<(String, i32), ParserError> {
    let mut input = input.into_iter();

    next_token!(input, |it| it.expect_func_keyword())?;

    let name = next_token!(input, |it| it.expect_identifier())?;

    next_token!(input, |it| it.expect_group())?;
    next_token!(input, |it| it.expect_arrow())?;

    let ty = next_token!(input, |it| it.expect_identifier())?;
    if ty != "I32" {
        return Err(TypeError);
    }

    let mut block = next_token!(input, |it| it.expect_block())?.into_iter();

    let number = next_token!(block, |it| it.expect_number_literal())?;

    end_tokens!(block);
    end_tokens!(input);

    Ok((name, number))
}
