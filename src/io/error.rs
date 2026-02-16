use crate::io::{GridsError, Span, read, write};

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    #[error("{0}")]
    Custom(String),

    #[error("Invalid version")]
    InvalidVersion,

    #[error("Grids errror: {0}")]
    Grids(#[from] GridsError),
}

pub type Result<T> = core::result::Result<T, Error>;

pub(crate) trait Context<T, E> {
    fn context<S: Into<String>>(self, context: S) -> core::result::Result<T, E>;
}

impl<T> Context<T, read::Error> for core::result::Result<T, Error> {
    fn context<S: Into<String>>(self, context: S) -> std::result::Result<T, read::Error> {
        self.map_err(|err| read::Error {
            kind: read::ErrorKind::Format(err),
            span: Span::default(),
            context: context.into(),
        })
    }
}

impl<T> Context<T, write::Error> for core::result::Result<T, Error> {
    fn context<S: Into<String>>(self, context: S) -> std::result::Result<T, write::Error> {
        self.map_err(|err| write::Error {
            kind: write::ErrorKind::Format(err),
            context: context.into(),
        })
    }
}
