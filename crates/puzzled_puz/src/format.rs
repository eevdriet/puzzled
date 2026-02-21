use crate::{Context, GridsError, Span, read, write};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid version")]
    InvalidVersion,

    #[error("Grids errror: {0}")]
    Grids(#[from] GridsError),

    #[error("Invalid clue specification: {reason}")]
    InvalidClueSpec { reason: String },

    // LTIM
    #[error("Invalid timer found: {reason}")]
    InvalidTimer { reason: String },

    // Text parsing
    #[error("Found invalid string literal \"{found}\", expected it to be delimited with \"...\"")]
    InvalidStringLiteral { found: String },

    #[error("Found invalid property \"{found}\": {reason}")]
    InvalidProperty { found: String, reason: String },

    #[error("Size of {kind} is overflowing (found {size}, expected <= {max_size})")]
    SizeOverflow {
        kind: String,
        size: usize,
        max_size: usize,
    },
}

pub type Result<T> = core::result::Result<T, Error>;

impl<T> Context<T, read::Error> for Result<T> {
    fn context<S: Into<String>>(self, context: S) -> read::Result<T> {
        self.map_err(|err| read::Error {
            kind: read::ErrorKind::Format(err),
            span: Span::default(),
            context: context.into(),
        })
    }
}

impl<T> Context<T, write::Error> for Result<T> {
    fn context<S: Into<String>>(self, context: S) -> write::Result<T> {
        self.map_err(|err| write::Error {
            kind: write::ErrorKind::Format(err),
            context: context.into(),
        })
    }
}
