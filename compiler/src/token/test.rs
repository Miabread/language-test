use super::{ScanError::*, TokenKind::*, *};

fn token(kind: TokenKind, start: usize, end: usize) -> Token {
    Token {
        kind,
        span: Span { start, end },
    }
}

fn no_error(tokens: Vec<Token>) -> (Vec<Token>, Vec<ScanError>) {
    (tokens, vec![])
}

#[test]
fn simple_tokens() {
    assert_eq!(
        scan("( ) { } , ;"),
        no_error(vec![
            token(OpenParen, 0, 0),
            token(CloseParen, 2, 2),
            token(OpenBrace, 4, 4),
            token(CloseBrace, 6, 6),
            token(Comma, 8, 8),
            token(Semicolon, 10, 10),
        ]),
    );
}

#[test]
fn comments() {
    assert_eq!(
        scan(
            r"foo // eeeeee
        bar"
        ),
        no_error(vec![
            token(Identifier("foo"), 0, 2),
            token(Identifier("bar"), 22, 24)
        ]),
    );
}

#[test]
fn strings() {
    assert_eq!(
        scan(r#" "foo" "bar" "baz "#),
        (
            vec![token(String("foo"), 1, 5), token(String("bar"), 7, 11)],
            vec![UnterminatedString { start: 13 }]
        ),
    );
}
