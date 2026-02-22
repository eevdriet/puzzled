use crate::{ColorError, GridError, StringError, TimerError, VersionError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Metadata
    #[error("Version error: {0}")]
    Version(#[from] VersionError),

    #[error("Timer error: {0}")]
    Timer(#[from] TimerError),

    #[error("Clue specification error: {reason}")]
    ClueSpec { reason: String },

    #[error("Found invalid property \"{found}\": {reason}")]
    InvalidProperty { found: String, reason: String },

    // Grids
    #[error("Grids error: {0}")]
    Grid(#[from] GridError),

    #[error("Color error: {0}")]
    Color(#[from] ColorError),

    #[error("String error: {0}")]
    String(#[from] StringError),

    #[error(
        "The solution grid has square '{solution_square}' at {row}R{col}C, while the state grid has '{state_square}' at that position"
    )]
    CellMismatch {
        solution_square: char,
        state_square: char,
        row: u8,
        col: u8,
    },
    #[error("Size of {kind} is overflowing (found {size}, expected <= {max_size})")]
    SizeOverflow {
        kind: String,
        size: usize,
        max_size: usize,
    },
}

pub type Result<T> = core::result::Result<T, Error>;
