use crate::{Context, format};

#[derive(Debug, thiserror::Error)]
#[error("Read error: {0}")]
pub enum Error {
    #[error("Read error")]
    Custom,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Format error: {0}")]
    Format(#[from] format::Error),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
}

pub type Result<T> = core::result::Result<T, Error>;

impl<T> Context<T, Error> for format::Result<T> {
    fn context<S: Into<String>>(self, _context: S) -> Result<T> {
        self.map_err(Error::Format)
    }
}

impl<T> Context<T, Error> for std::io::Result<T> {
    fn context<S: Into<String>>(self, _context: S) -> Result<T> {
        self.map_err(Error::Io)
    }
}
