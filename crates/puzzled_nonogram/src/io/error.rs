use crate::io::img;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[cfg(feature = "puz")]
    #[error("Puz error: {0}")]
    Puz(#[from] puzzled_puz::read::Error),

    #[cfg(feature = "image")]
    #[error("Image error: {0}")]
    Image(#[from] img::Error),

    #[error("Tried to parse nonogram from file with unsupported extension '{0}'")]
    UnsupportedExtension(String),
}

pub type Result<T> = core::result::Result<T, Error>;
