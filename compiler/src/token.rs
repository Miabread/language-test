#[derive(Debug, Clone)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub start: usize,
    pub end: usize,
}

// Note: variant order should be the same as match order in scan()
#[derive(Debug, Clone)]
pub enum TokenKind<'src> {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Semicolon,
    Number(f64),
    String(&'src str),
    FuncKeyword,
    DebugKeyword,
    Identifier(&'src str),
}

#[derive(Debug, Clone)]
pub enum ScanError {
    UnterminatedString { start: usize },
    UnknownCharacter { position: usize },
}

pub fn scan(source: &str) -> Result<Vec<Token<'_>>, Vec<ScanError>> {
    let mut chars = source.char_indices().peekable();
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    'outer: while let Some(char) = chars.next() {
        let kind = match char.1 {
            // Ignore whitespace in between token boundaries
            char if char.is_whitespace() => continue,

            // Single char tokens get handled outside the match
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,

            // Check for "//" comment starter
            '/' if chars.peek().map(|c| c.1 == '/').unwrap_or(false) => {
                // Keep consuming chars until a new line
                while let Some(char) = chars.next() {
                    if char.1 == '\n' {
                        break;
                    }
                }
                continue;
            }

            // Parse number literals
            number if number.is_ascii_digit() => {
                // Last digit needed for slice
                let mut last = char;

                // Keep consuming digit chars
                for char in &mut chars {
                    if char.1.is_ascii_digit() {
                        last = char;
                    } else {
                        break;
                    }
                }

                // TODO: Implement float literals. Delayed due to needing double look ahead.

                // Parse the digits into a number
                let kind = TokenKind::Number(
                    source[char.0..=last.0]
                        .parse()
                        .expect("parse to work since we only pass digits"),
                );

                tokens.push(Token {
                    start: char.0,
                    end: last.0,
                    kind,
                });
                continue;
            }

            // Parse string literals
            '"' => {
                // Keep consuming chars until a quote
                let closing = loop {
                    match chars.next() {
                        Some(closing @ (_, '"')) => break closing,
                        None => {
                            errors.push(ScanError::UnterminatedString { start: char.0 });
                            break 'outer;
                        }
                        _ => {}
                    }
                };

                // Slice the leading and trailing quote
                let kind = TokenKind::String(&source[char.0 + 1..closing.0]);

                tokens.push(Token {
                    start: char.0,
                    end: closing.0,
                    kind,
                });
                continue;
            }

            letter if letter.is_alphabetic() => {
                // Last digit needed for slice
                let mut last = char;

                // Keep consuming digit chars
                for char in &mut chars {
                    if char.1.is_alphanumeric() {
                        last = char;
                    } else {
                        break;
                    }
                }

                // Check if reserved word
                let kind = match &source[char.0..=last.0] {
                    "func" => TokenKind::FuncKeyword,
                    "debug" => TokenKind::DebugKeyword,
                    ident => TokenKind::Identifier(ident),
                };

                tokens.push(Token {
                    start: char.0,
                    end: last.0,
                    kind,
                });
                continue;
            }

            // Handle unknown char
            _ => {
                errors.push(ScanError::UnknownCharacter { position: char.0 });
                continue;
            }
        };

        // All single char tokens are handled down here
        tokens.push(Token {
            start: char.0,
            end: char.0,
            kind,
        })
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}
