use crate::error::{TapeError, TapeResult};
use std::io::ErrorKind;
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// An Input reader to asynchronously read a type
/// that implements AsyncBufRead and AsyncSeek.
pub struct InputReader {
    inner: Box<dyn AsyncBufRead + Unpin + Send>,
    buf: String,
    index: usize,
}

impl InputReader {
    /// Creates a new Input Reader
    pub fn new<T: AsyncBufRead + Unpin + Send + 'static>(inner: T) -> Self {
        Self {
            inner: Box::new(inner),
            buf: String::new(),
            index: 0,
        }
    }

    /// Reads the next char consuming it in the process
    #[inline]
    pub async fn consume(&mut self) -> TapeResult<char> {
        self.seek_to(self.index + 1).await?;

        self.read_current().await
    }

    /// Returns the next char without forwarding
    #[inline]
    pub async fn peek(&mut self) -> TapeResult<char> {
        self.seek_to(self.index + 1).await?;
        let char = self.read_current().await?;
        self.seek_to(self.index - 1).await?;

        Ok(char)
    }

    /// Returns if EOF has been reached
    #[inline]
    pub async fn check_eof(&mut self) -> bool {
        if let Err(TapeError::EOF) = self.peek().await {
            true
        } else {
            false
        }
    }

    /// Returns the previous char
    #[inline]
    pub async fn previous(&mut self) -> Option<char> {
        self.read_current().await.ok()
    }

    /// Seeks to a given index
    pub async fn seek_to(&mut self, to_index: usize) -> TapeResult<()> {
        while to_index >= self.buf.len() {
            let mut line = String::new();
            self.inner.read_line(&mut line).await.map_err(|e| {
                if e.kind() == ErrorKind::UnexpectedEof {
                    TapeError::EOF
                } else {
                    TapeError::TokioIoError(e)
                }
            })?;
            if line.is_empty() {
                break;
            }
            self.buf.push_str(&line);
        }
        self.index = to_index;

        Ok(())
    }

    /// Returns the current index
    pub fn index(&self) -> usize {
        self.index
    }

    /// Reads the next char returning \x00 for EOF
    async fn read_current(&mut self) -> TapeResult<char> {
        if self.index == 0 {
            return Err(TapeError::IndexError);
        }
        let result = self
            .buf
            .get(self.index - 1..self.index)
            .ok_or(TapeError::EOF)?
            .chars()
            .next()
            .ok_or(TapeError::EOF);

        result
    }
}
