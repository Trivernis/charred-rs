use crate::error::TapeResult;
use crate::traits::AsyncReadSeek;
use std::io::SeekFrom;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::io::{AsyncSeekExt, ErrorKind};

/// An Input reader to asynchronously read a type
/// that implements AsyncBufRead and AsyncSeek.
pub struct InputReader {
    inner: BufReader<Box<dyn AsyncReadSeek>>,
}

impl InputReader {
    pub fn new<T: AsyncReadSeek + 'static>(inner: T) -> Self {
        Self {
            inner: BufReader::new(Box::new(inner)),
        }
    }

    /// Reads the next char consuming it in the process
    pub async fn consume(&mut self) -> TapeResult<char> {
        self.read_next().await
    }

    /// Returns the next char without forwarding
    pub async fn peek(&mut self) -> TapeResult<char> {
        let char = self.read_next().await?;
        self.inner.get_mut().seek(SeekFrom::Current(-1)).await?;

        Ok(char)
    }

    /// Returns if EOF has been reached
    pub async fn check_eof(&mut self) -> TapeResult<bool> {
        let char = self.read_next().await?;

        Ok(char == '\x00')
    }

    /// Reads the next char returning \x00 for EOF
    async fn read_next(&mut self) -> TapeResult<char> {
        let mut value = String::with_capacity(1);

        match self.inner.read_to_string(&mut value).await {
            Ok(_) => Ok(value.chars().next().unwrap()),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => Ok('\x00'),
            Err(e) => Err(e.into()),
        }
    }
}
