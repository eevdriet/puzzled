use crate::io::{format, read, write};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Read(#[from] read::Error),

    #[error("{0}")]
    Write(#[from] write::Error),

    #[error("{0}")]
    Format(#[from] format::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

pub(crate) trait Context<T, E> {
    fn context<S: Into<String>>(self, context: S) -> core::result::Result<T, E>;
}
