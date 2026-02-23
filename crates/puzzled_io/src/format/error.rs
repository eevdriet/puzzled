use puzzled_core::{ColorError, GridError, TimerError, VersionError};

use crate::format::StringError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // General format errors
    #[error("Version error: {0}")]
    Version(#[from] VersionError),

    #[error("Timer error: {0}")]
    Timer(#[from] TimerError),

    #[error("Grids error: {0}")]
    Grid(#[from] GridError),

    #[error("Color error: {0}")]
    Color(#[from] ColorError),

    #[error("String error: {0}")]
    String(#[from] StringError),

    // Other general errors
    #[error("Size of {kind} is overflowing (found {size}, expected <= {max_size})")]
    SizeOverflow {
        kind: String,
        size: usize,
        max_size: usize,
    },

    // Puzzle specific errors
    #[error("{0}")]
    PuzzleSpecific(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub type Result<T> = core::result::Result<T, Error>;
