use std::str::Utf8Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Utf8Error(#[from] Utf8Error),

    // Text parsing
    #[error("Found invalid string literal \"{found}\", expected it to be delimited with \"...\"")]
    InvalidLiteral { found: String },
}
