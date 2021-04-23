use crate::error::TapeResult;
use crate::input_reader::InputReader;
use crate::token::{ProtoToken, Token};
use async_trait::async_trait;
use std::io::Cursor;

#[derive(Debug)]
struct TestToken(i32);

#[async_trait]
impl ProtoToken for TestToken {
    async fn try_parse(reader: &mut InputReader) -> TapeResult<Option<Token>> {
        let mut num = String::new();
        while !reader.check_eof().await && reader.peek().await?.is_numeric() {
            num.push(reader.consume().await?);
        }
        if num.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Token::new(TestToken(num.parse::<i32>().unwrap()))))
        }
    }
}

#[tokio::test]
async fn it_parses() {
    let mut reader = InputReader::new(Cursor::new("128"));
    let token = TestToken::try_parse(&mut reader).await.unwrap();
    assert!(token.is_some());
    let token = token.unwrap().try_into::<TestToken>().unwrap();
    assert_eq!(token.0, 128);

    let mut reader = InputReader::new(Cursor::new("string a12 24\n"));
    let token = TestToken::try_parse(&mut reader).await.unwrap();
    assert!(token.is_none());
    reader.seek_to(8).await.unwrap();

    let token = TestToken::try_parse(&mut reader).await.unwrap();
    assert!(token.is_some());
    let token = token.unwrap().try_into::<TestToken>().unwrap();
    assert_eq!(token.0, 12);
}

#[test]
fn it_converts() {
    let token = Token::new(TestToken(12));
    assert!(token.is::<TestToken>());

    let test_token = token.try_as::<TestToken>();
    assert!(test_token.is_some());
    assert_eq!(test_token.unwrap().0, 12);

    let test_token = token.try_into::<TestToken>();
    assert!(test_token.is_some());
    assert_eq!(test_token.unwrap().0, 12);
}
