use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("Nonogram error: {0}")]
    Nonogram(#[from] puzzled_nono::Error),

    #[error("Loading error: {0}")]
    Load(#[from] puzzled_nono::io::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
