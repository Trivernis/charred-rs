use crate::error::{TapeError, TapeResult};
use std::io::ErrorKind;
use tokio::io::{AsyncBufRead, AsyncBufReadExt};

/// An Input reader to asynchronously read a type
/// that implements AsyncBufRead and AsyncSeek.
pub struct InputReader {
    inner: Box<dyn AsyncBufRead + Unpin>,
    buf: String,
    index: usize,
}

impl InputReader {
    pub fn new<T: AsyncBufRead + Unpin + 'static>(inner: T) -> Self {
        Self {
            inner: Box::new(inner),
            buf: String::new(),
            index: 0,
        }
    }

    /// Reads the next char consuming it in the process
    #[inline]
    pub async fn consume(&mut self) -> TapeResult<char> {
        self.read_next().await
    }

    /// Returns the next char without forwarding
    #[inline]
    pub async fn peek(&mut self) -> TapeResult<char> {
        let char = self.read_next().await?;
        self.seek_to(self.index - 1).await?;

        Ok(char)
    }

    /// Returns if EOF has been reached
    #[inline]
    pub async fn check_eof(&mut self) -> bool {
        if let Err(TapeError::EOF) = self.read_next().await {
            true
        } else {
            false
        }
    }

    /// Reads the next char returning \x00 for EOF
    async fn read_next(&mut self) -> TapeResult<char> {
        self.seek_to(self.index + 1).await?;
        let result = self
            .buf
            .get(self.index - 1..self.index)
            .ok_or(TapeError::EOF)?
            .chars()
            .next()
            .ok_or(TapeError::EOF);

        result
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
}
