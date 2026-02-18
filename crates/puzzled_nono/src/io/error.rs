use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("Nonogram error: {0}")]
    Nonogram(#[from] nono::Error),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Image error: {0}")]
    Img(#[from] image::ImageError),

    #[error("Tried to parse nonogram from file with unsupported extension '{0}'")]
    UnsupportedExtension(String),
}

pub type Result<T> = core::result::Result<T, Error>;
