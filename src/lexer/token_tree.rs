use crate::parser::ParserError;

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
pub enum TokenTreeType {
    FuncKeyword,
    Arrow,
    Identifier,
    NumberLiteral,
    Group,
    Block,
    Error,
}

impl From<TokenTree> for TokenTreeType {
    fn from(other: TokenTree) -> Self {
        match other {
            TokenTree::FuncKeyword => TokenTreeType::FuncKeyword,
            TokenTree::Arrow => TokenTreeType::Arrow,
            TokenTree::Identifier(_) => TokenTreeType::Identifier,
            TokenTree::NumberLiteral(_) => TokenTreeType::NumberLiteral,
            TokenTree::Group(_) => TokenTreeType::Group,
            TokenTree::Block(_) => TokenTreeType::Block,
            TokenTree::Error => TokenTreeType::Error,
        }
    }
}

impl TokenTree {
    pub fn expect_func_keyword(self) -> Result<(), ParserError> {
        match self {
            Self::FuncKeyword => Ok(()),
            _ => Err(ParserError::InvalidToken {
                expected: TokenTreeType::FuncKeyword,
                found: self,
            }),
        }
    }

    pub fn expect_arrow(self) -> Result<(), ParserError> {
        match self {
            Self::Arrow => Ok(()),
            _ => Err(ParserError::InvalidToken {
                expected: TokenTreeType::Arrow,
                found: self,
            }),
        }
    }

    pub fn expect_identifier(self) -> Result<String, ParserError> {
        match self {
            Self::Identifier(it) => Ok(it),
            _ => Err(ParserError::InvalidToken {
                expected: TokenTreeType::Identifier,
                found: self,
            }),
        }
    }

    pub fn expect_number_literal(self) -> Result<i32, ParserError> {
        match self {
            Self::NumberLiteral(it) => Ok(it),
            _ => Err(ParserError::InvalidToken {
                expected: TokenTreeType::NumberLiteral,
                found: self,
            }),
        }
    }

    pub fn expect_group(self) -> Result<Vec<TokenTree>, ParserError> {
        match self {
            Self::Group(it) => Ok(it),
            _ => Err(ParserError::InvalidToken {
                expected: TokenTreeType::Group,
                found: self,
            }),
        }
    }

    pub fn expect_block(self) -> Result<Vec<TokenTree>, ParserError> {
        match self {
            Self::Block(it) => Ok(it),
            _ => Err(ParserError::InvalidToken {
                expected: TokenTreeType::Block,
                found: self,
            }),
        }
    }
}
