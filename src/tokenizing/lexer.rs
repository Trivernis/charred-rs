use crate::tokenizing::{EOFToken, Token, TokenCheckerFn, UnknownToken};
use crate::InputReader;
use crate::TapeResult;

pub struct Lexer {
    reader: InputReader,
    checkers: Vec<TokenCheckerFn>,
}

impl Lexer {
    /// Creates a new lexer with provided checker functions
    pub fn new(reader: InputReader, checkers: Vec<TokenCheckerFn>) -> Self {
        Self { reader, checkers }
    }

    /// Scans for tokenizing
    pub async fn scan(&mut self) -> TapeResult<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.reader.check_eof().await {
            let index = self.reader.index();
            let mut found = false;

            for checker_fn in &self.checkers {
                if let Some(token) = checker_fn.as_ref()(&mut self.reader).await? {
                    tokens.push(token);
                    found = true;
                    break;
                } else {
                    self.reader.seek_to(index).await?;
                }
            }
            if !found {
                tokens.push(Token::new(UnknownToken(self.reader.consume().await?)))
            }
        }
        tokens.push(Token::new(EOFToken));

        Ok(tokens)
    }
}
