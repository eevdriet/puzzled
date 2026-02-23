#[cfg(feature = "puz")]
use crate::puz;

#[cfg(feature = "image")]
use crate::image;
use crate::text;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Text error: {0}")]
    Text(#[from] text::Error),

    #[cfg(feature = "puz")]
    #[error("Puz error: {0}")]
    Puz(#[from] puz::Error),

    #[cfg(feature = "image")]
    #[error("Image error: {0}")]
    Image(#[from] image::Error),

    #[error("Tried to parse nonogram from file with unsupported extension '{0}'")]
    UnsupportedExtension(String),
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Context<T, E> {
    fn context<S>(self, context: S) -> std::result::Result<T, E>
    where
        S: Into<String>;
}
