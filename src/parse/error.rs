use thiserror::Error;

use crate::parse::{ExtrasError, GridError, HeaderError, ReadError, Span};

#[derive(Debug, Error, Clone)]
#[cfg_attr(
    not(feature = "miette"),
    error("{kind} while parsing '{context}' at {span:?}")
)]
#[cfg_attr(feature = "miette", error("{kind} while parsing '{context}'"))]
pub struct Error {
    /// Where the error occurred
    pub span: Span,
    /// What kind of error occurred
    pub kind: ErrorKind,
    /// Context for what was currently parsed when the error occurred
    pub context: String,
}

#[derive(Debug, Error, Clone)]
pub enum ErrorKind {
    #[error("{0}")]
    Custom(String),

    #[error("{0}")]
    Read(#[from] ReadError),

    #[error("{0}")]
    Header(#[from] HeaderError),

    #[error("{0}")]
    Puzzle(#[from] GridError),

    #[error("{0}")]
    Extras(#[from] ExtrasError),

    #[error("Invalid checksum '{found}' found, expected '{expected}'")]
    InvalidChecksum { found: u16, expected: u16 },

    #[error("Expected to find {expected} checksums, found {found}")]
    MissingChecksum { found: usize, expected: usize },

    #[error("Cannot place clue #{id}: {clue}")]
    MissingClue { id: u16, clue: String },
}

pub type Result<T> = core::result::Result<T, Error>;

/// [Errors](struct@Error) that can be recovered from when parsing in non-strict mode
///
/// For [parsing](crate::Parser), these include
/// - Ignoring [invalid checksums](crate::parse#validating-checksums)
/// - Ignoring [invalid extra sections](crate::parse#extra-sections)
pub type Warning = Error;

#[cfg(feature = "miette")]
mod miette {
    use crate::parse::Error;
    use miette::{Diagnostic, LabeledSpan};

    impl Diagnostic for Error {
        fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
            let span = LabeledSpan::at(self.span.clone(), "here");

            Some(Box::new(std::iter::once(span)))
        }
    }
}
