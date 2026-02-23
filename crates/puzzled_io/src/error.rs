#[cfg(feature = "puz")]
use crate::puz;

#[cfg(feature = "image")]
use crate::image;
use crate::text;

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("Text error: {0}")]
    Text(#[from] text::read::Error),

    #[cfg(feature = "puz")]
    #[error("Puz error: {0}")]
    Puz(#[from] puz::read::Error),

    #[cfg(feature = "image")]
    #[error("Image error: {0}")]
    Image(#[from] image::read::Error),

    #[error("Cannot read puzzle from unsupported format '{format}'")]
    UnsupportedFormat { format: String },
}

#[derive(Debug, thiserror::Error)]
pub enum WritError {
    #[error("Text error: {0}")]
    Text(#[from] text::read::Error),

    #[cfg(feature = "puz")]
    #[error("Puz error: {0}")]
    Puz(#[from] puz::read::Error),

    #[cfg(feature = "image")]
    #[error("Image error: {0}")]
    Image(#[from] image::read::Error),

    #[error("Cannot write puzzle with unsupported format '{format}'")]
    UnsupportedFormat { format: String },
}

pub trait Context<T, E> {
    fn context<S>(self, context: S) -> std::result::Result<T, E>
    where
        S: Into<String>;
}
