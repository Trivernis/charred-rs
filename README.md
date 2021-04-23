# Charred

This crate provides a generic asynchronous lexer that operates on files with tokio.
Tokens are parsed with provided async closures.

## Usage

```rust
use crate::error::TapeResult;
use crate::input_reader::InputReader;
use crate::lexer::Lexer;
use crate::token::{Token, TokenCheckerFn};
use std::io::Cursor;
use std::sync::Arc;

struct NumberToken(i32);
struct StringToken(String);
struct WhiteSpaceToken;

async fn parse_number_token(reader: &mut InputReader) -> TapeResult<Option<Token>> {
    let mut num = String::new();
    while !reader.check_eof().await && reader.peek().await?.is_numeric() {
        num.push(reader.consume().await?);
    }
    if num.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Token::new(NumberToken(num.parse::<i32>().unwrap()))))
    }
}

async fn parse_whitespace_token(reader: &mut InputReader) -> TapeResult<Option<Token>> {
    let mut count = 0;
    while !reader.check_eof().await && reader.peek().await?.is_whitespace() {
        reader.consume().await?;
        count += 1;
    }
    if count > 0 {
        Ok(Some(Token::new(WhiteSpaceToken)))
    } else {
        Ok(None)
    }
}

async fn parse_string_token(reader: &mut InputReader) -> TapeResult<Option<Token>> {
    let mut value = String::new();
    while !reader.check_eof().await
        && !reader.peek().await?.is_numeric()
        && !reader.peek().await?.is_whitespace()
    {
        value.push(reader.consume().await?);
    }
    if value.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Token::new(StringToken(value))))
    }
}

#[tokio::main]
async fn main() {
    // functions that try to parse the token into an object
    let checkers: Vec<TokenCheckerFn> = vec![
        Arc::new(|reader| Box::pin(parse_number_token(reader))),
        Arc::new(|reader| Box::pin(parse_whitespace_token(reader))),
        Arc::new(|reader| Box::pin(parse_string_token(reader))),
    ];
    // input reader encapsulates (almost) any type that implements AsyncBufRead
    let input_reader = InputReader::new(Cursor::new("Word 12"));
    let mut lexer = Lexer::new(input_reader, checkers);

    // scan starts scanning the provided input
    let tokens = lexer.scan().await.unwrap();
    assert!(!tokens.is_empty());

    let mut tokens = tokens.into_iter();
    // use the is, try_as and try_into methods on the token type to get the underlying value
    assert!(tokens.next().unwrap().is::<StringToken>());
    assert!(tokens.next().unwrap().is::<WhiteSpaceToken>());
    assert!(tokens.next().unwrap().is::<NumberToken>());
}
```

## License

Apache-2.0
