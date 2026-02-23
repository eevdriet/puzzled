use crate::image::{read, write};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Read error: {0}")]
    Read(#[from] read::Error),

    #[error("Write error: {0}")]
    Write(#[from] write::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
