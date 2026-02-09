use thiserror::Error;

use crate::{ExtrasError, GridError, HeaderError, ReadError};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Found I/O error: {0}")]
    Read(#[from] ReadError),

    #[error("Found error while parsing header: {0}")]
    Header(#[from] HeaderError),

    #[error("Found error while parsing puzzle grid: {0}")]
    Puzzle(#[from] GridError),

    #[error("Found error while parsing extra sections: {0}")]
    Extras(#[from] ExtrasError),

    #[error("Invalid {kind} checksum {found} found, expected {expected}")]
    InvalidChecksum {
        kind: String,
        found: u16,
        expected: u16,
    },

    #[error("Expected to find {expected} {kind} checksums, found {found}")]
    MissingChecksum {
        kind: String,
        found: usize,
        expected: usize,
    },
}

pub type Result<T> = core::result::Result<T, Error>;
