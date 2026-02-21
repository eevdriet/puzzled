use puzzled_core::GridError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid version: {reason}")]
    InvalidVersion { reason: String },

    #[error("Grids error: {0}")]
    Grids(#[from] GridError),

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
}

pub type Result<T> = std::result::Result<T, Error>;
