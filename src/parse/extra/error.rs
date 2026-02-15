use thiserror::Error;

use crate::Position;

#[derive(Debug, Error, Clone)]
pub enum ExtrasError {
    // General
    #[error(
        "Read invalid section header {found}, expected one of 'GRBS', 'RTBL', 'LTIM' or 'GTEXT'"
    )]
    InvalidSection { found: String },

    // GRBS
    #[error("Expected RTBL to include rebus #{rebus} at position {pos:?}, but not found")]
    MissingRebus { pos: Position, rebus: u8 },

    // RTBL
    #[error("Rebus #{square} in the RTBL is invalid: {reason}")]
    InvalidRebus { square: u16, reason: String },

    // LTIM
    #[error("Invalid timer found: {reason}")]
    InvalidTimer { reason: String },

    // GEXT
    #[error(
        "Encountered invalid bitmask {mask} at position {pos:?} in GEXT, only the following bits should be set: 0x10, 0x20, 0x40 and 0x80"
    )]
    InvalidBitmask { pos: Position, mask: u8 },
}
