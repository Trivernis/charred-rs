use crate::token::{EOFToken, Token};

pub struct TokenReader {
    tokens: Vec<Token>,
    index: usize,
    eof: Token,
}

impl TokenReader {
    /// Creates a new token reader
    pub fn new(mut tokens: Vec<Token>) -> Self {
        if tokens.last().is_none() || !tokens.last().unwrap().is::<EOFToken>() {
            // ensure that the last token always is an EOF Token
            tokens.push(Token::new(EOFToken));
        }
        Self {
            tokens,
            index: 0,
            eof: Token::new(EOFToken),
        }
    }

    /// Peeks the next token
    #[inline]
    pub fn peek(&self) -> &Token {
        self.tokens.get(self.index).unwrap_or(&self.eof)
    }

    /// Checks if the next token is of a specific type without consuming it
    #[inline]
    pub fn peek_is<T: 'static>(&self) -> bool {
        self.peek().is::<T>()
    }

    /// Peeks the next token and tries to return is as a concrete type
    #[inline]
    pub fn peek_as<T: 'static>(&self) -> Option<&T> {
        self.peek().try_as::<T>()
    }

    /// Consumes the next token and returns it
    pub fn consume(&mut self) -> &Token {
        self.index += 1;
        self.tokens.get(self.index - 1).unwrap_or(&self.eof)
    }

    /// Consumes the next token and tries to return it as the specified type
    #[inline]
    pub fn consume_as<T: 'static>(&mut self) -> Option<&T> {
        self.consume().try_as::<T>()
    }

    /// Seeks to the given index
    #[inline]
    pub fn seek(&mut self, to_index: usize) {
        self.index = to_index
    }

    /// Returns if EOF has been reached
    #[inline]
    pub fn check_eof(&self) -> bool {
        self.peek_is::<EOFToken>()
    }
}
