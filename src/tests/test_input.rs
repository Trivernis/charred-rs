use crate::error::{TapeError, TapeResult};
use crate::input_reader::InputReader;
use std::io::Cursor;

fn get_reader() -> InputReader {
    let data = "ABCDEFG HIJKLMNOP 12345567890\nSecond Line\n\n";
    InputReader::new(Cursor::new(data))
}

#[tokio::test]
async fn it_peeks() {
    let mut reader = get_reader();
    assert_eq!(reader.peek().await.unwrap(), 'A');
    assert_eq!(reader.peek().await.unwrap(), 'A');
    assert_eq!(reader.peek().await.unwrap(), 'A');
}

#[tokio::test]
async fn it_consumes() {
    let mut reader = get_reader();
    assert_eq!(reader.consume().await.unwrap(), 'A');
    assert_eq!(reader.consume().await.unwrap(), 'B');
    assert_eq!(reader.consume().await.unwrap(), 'C');
}

#[tokio::test]
async fn it_checks_for_eof() {
    let mut reader = get_reader();
    assert!(!is_eof(reader.seek_to(29).await));
    assert!(!reader.check_eof().await);
    assert!(!is_eof(reader.seek_to(47).await));
    assert!(is_eof(reader.consume().await.map(|_| ())));
    assert!(reader.check_eof().await);
}

fn is_eof(result: TapeResult<()>) -> bool {
    match result {
        Err(TapeError::EOF) => true,
        _ => false,
    }
}
