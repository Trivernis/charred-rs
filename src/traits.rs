use tokio::io::{AsyncRead, AsyncSeek};

pub trait AsyncReadSeek: AsyncRead + AsyncSeek + Unpin {}
