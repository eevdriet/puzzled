use puzzled_core::GridError;

use crate::{Context, Span, read, write};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Header
    #[error("Invalid version: {reason}")]
    InvalidVersion { reason: String },

    // Grids
    #[error("Grids error: {0}")]
    Grids(#[from] GridError),

    #[error(
        "The solution grid has square '{solution_square}' at {row}R{col}C, while the state grid has '{state_square}' at that position"
    )]
    CellMismatch {
        solution_square: char,
        state_square: char,
        row: u8,
        col: u8,
    },

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
