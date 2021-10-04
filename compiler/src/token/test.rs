use super::{ScanError::*, TokenKind::*, *};
use std::ops::Range;

fn token(kind: TokenKind, Range { start, end }: Range<usize>) -> ScanResult {
    Ok(Token {
        kind,
        span: Span { start, end },
    })
}

#[test]
fn simple_tokens() {
    assert_eq!(
        scan("( ) { } -> , ;").collect_vec(),
        vec![
            token(OpenParen, 0..0),
            token(CloseParen, 2..2),
            token(OpenBrace, 4..4),
            token(CloseBrace, 6..6),
            token(Arrow, 8..9),
            token(Comma, 11..11),
            token(Semicolon, 13..13),
        ]
    );
}

#[test]
fn comments() {
    assert_eq!(
        scan(
            r"foo // eeeeee
        bar"
        )
        .collect_vec(),
        vec![
            token(Identifier("foo"), 0..2),
            token(Identifier("bar"), 22..24)
        ]
    );
}

#[test]
fn strings() {
    assert_eq!(
        scan(r#" "foo" "bar" "baz "#).collect_vec(),
        vec![
            token(String("foo"), 1..5),
            token(String("bar"), 7..11),
            Err(UnterminatedString { start: 13 })
        ],
    );
}

#[test]
fn comments_whitespace() {
    assert_eq!(
        scan(
            r"123; aaa // wew lad
// wew lad
            // wew lad
    // wew lad
bbb ccc   // foo bar
456; 789 // baz"
        )
        .collect_vec(),
        vec![
            token(Integer(123), 0..2),
            token(Semicolon, 3..3),
            token(Identifier("aaa"), 5..7),
            token(Identifier("bbb"), 69..71),
            token(Identifier("ccc"), 73..75),
            token(Integer(456), 90..92),
            token(Semicolon, 93..93),
            token(Integer(789), 95..97),
        ]
    )
}

#[test]
fn comments_and_slash() {
    assert_eq!(
        scan(r"/    // wew").collect_vec(),
        vec![Err(InvalidToken { position: 0 })]
    );
}

#[test]
fn character_literals() {
    assert_eq!(
        scan(r#"'w' '' 'wew  '"' '"#).collect_vec(),
        vec![
            token(Character('w'), 0..2),
            Err(EmptyCharacter {
                span: Span { start: 4, end: 5 }
            }),
            Err(CharacterExpectedClosing {
                actual: 'e',
                span: Span { start: 7, end: 9 }
            }),
            token(Identifier("w"), 10..10),
            token(Character('\"'), 13..15),
            Err(UnterminatedCharacterEof { start: 17 })
        ]
    );
}
