use crate::format;

#[derive(Debug, thiserror::Error)]
#[error("Read error: {0}")]
pub enum Error {
    #[error("Read error")]
    Custom(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Format error: {0}")]
    Format(#[from] format::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
