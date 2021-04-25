use crate::do_match;
use crate::matching::{match_many, match_many_mul, match_many_mul_plus, match_one, MatchResult};
use crate::tokenizing::TokenReader;
use crate::tokenizing::{EOFToken, Token};
use std::any::TypeId;

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

#[tokio::test]
async fn it_works() {
    let mut reader = get_reader();
    assert!(match_one::<AToken>(&mut reader).await.unwrap().is_some());

    assert!(match_many(
        &mut reader,
        &[TypeId::of::<AToken>(), TypeId::of::<BToken>()],
    )
    .await
    .unwrap()
    .is_some());

    assert_eq!(
        match_many_mul(
            &mut reader,
            &[TypeId::of::<AToken>(), TypeId::of::<CToken>()],
        )
        .await
        .unwrap()
        .unwrap(),
        3
    );

    assert!(match_one::<EOFToken>(&mut reader).await.unwrap().is_some())
}

#[tokio::test]
async fn test_macro_matching() {
    let mut reader = get_reader();
    assert!(match_with_macro(&mut reader).await.unwrap().is_some());
}

async fn match_with_macro(reader: &mut TokenReader) -> MatchResult<()> {
    do_match!(match_one::<AToken>(reader));
    do_match!(match_many(
        reader,
        &[TypeId::of::<AToken>(), TypeId::of::<BToken>()]
    ));
    do_match!(match_many_mul_plus(
        reader,
        &[TypeId::of::<CToken>(), TypeId::of::<AToken>()]
    ));
    do_match!(match_one::<EOFToken>(reader));

    Ok(Some(()))
}
