use crate::{Context, format};

#[derive(Debug, thiserror::Error)]
#[error("Write error: {0}")]
pub enum Error {
    #[error("Write error")]
    Custom(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Format error: {0}")]
    Format(#[from] format::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl<T> Context<T, Error> for format::Result<T> {
    fn context<S: Into<String>>(self, _context: S) -> Result<T> {
        self.map_err(Error::Format)
    }
}
