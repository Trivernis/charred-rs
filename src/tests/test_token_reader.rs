use crate::token::{EOFToken, Token};
use crate::token_reader::TokenReader;

struct AToken;
struct BToken;
struct CToken;

fn get_reader() -> TokenReader {
    let tokens = vec![
        Token::new(AToken),
        Token::new(BToken),
        Token::new(AToken),
        Token::new(CToken),
        Token::new(CToken),
        Token::new(EOFToken),
    ];

    TokenReader::new(tokens)
}

#[test]
fn peek_does_not_consume() {
    let reader = get_reader();

    assert!(reader.peek_is::<AToken>());
    assert!(!reader.peek_is::<BToken>());
    assert!(reader.peek_is::<AToken>());
}

#[test]
fn consume_does_consume() {
    let mut reader = get_reader();
    assert!(reader.consume_as::<AToken>().is_some());
    assert!(reader.consume_as::<BToken>().is_some());
    assert!(reader.consume_as::<AToken>().is_some());
    assert!(reader.consume_as::<CToken>().is_some());
}

#[test]
fn check_eof_works() {
    let mut reader = get_reader();
    reader.seek(4);
    assert!(!reader.check_eof());
    reader.seek(5);
    assert!(reader.check_eof());
}
#[test]
fn peek_and_consume_returns_eof_on_input_end() {
    let mut reader = get_reader();
    reader.seek(4);
    assert!(reader.consume_as::<EOFToken>().is_none());
    assert!(reader.consume_as::<EOFToken>().is_some());
    assert!(reader.consume_as::<EOFToken>().is_some());
    assert!(reader.consume_as::<EOFToken>().is_some());
    reader.seek(0);
    assert!(reader.consume_as::<EOFToken>().is_none());
}
