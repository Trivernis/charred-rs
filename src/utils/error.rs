use thiserror::Error;

pub type TapeResult<T> = Result<T, TapeError>;

#[derive(Debug, Error)]
pub enum TapeError {
    #[error("IO Error: {0}")]
    TokioIoError(#[from] tokio::io::Error),

    #[error("Unexpected EOF")]
    EOF,

    #[error("Failed to read char at current index")]
    IndexError,
}
