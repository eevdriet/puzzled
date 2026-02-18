use puzzled_core::Position;
use thiserror::Error;

use crate::io::{FILE_MAGIC, Span, error::Context, format};

#[derive(Debug, Error)]
#[cfg_attr(
    not(feature = "miette"),
    error("{kind} while reading '{context}' at {span:?}")
)]
#[cfg_attr(feature = "miette", error("{kind} while reading '{context}'"))]
pub struct Error {
    /// Where the error occurred
    pub span: Span,
    /// What kind of error occurred
    pub kind: ErrorKind,
    /// Context for what was currently parsed when the error occurred
    pub context: String,
}

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("I/O error: {0}")]
    Io(std::io::Error),

    #[error("{0}")]
    Format(#[from] format::Error),

    #[error("Invalid file magic: .puz files expect '{FILE_MAGIC}', but found '{found}'")]
    InvalidFileMagic { found: String },

    #[error("Invalid checksum '{found}' found, expected '{expected}'")]
    InvalidChecksum { found: u16, expected: u16 },

    #[error("Expected to find {expected} checksums, found {found}")]
    MissingChecksum { found: usize, expected: usize },

    #[error("Cannot place clue #{id}: {clue}")]
    MissingClue { id: u16, clue: String },

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

    // GEXT
    #[error(
        "Encountered invalid bitmask {mask} at position {pos:?} in GEXT, only the following bits should be set: 0x10, 0x20, 0x40 and 0x80"
    )]
    InvalidCellStyle { pos: Position, mask: u8 },
}

impl<T> Context<T, Error> for std::io::Result<T> {
    fn context<S: Into<String>>(self, context: S) -> Result<T> {
        self.map_err(|err| Error {
            span: Span::default(),
            kind: ErrorKind::Io(err),
            context: context.into(),
        })
    }
}

pub type Result<T> = core::result::Result<T, Error>;

/// [Errors](struct@Error) that can be recovered from when reading in non-strict mode
pub type Warning = Error;

#[cfg(feature = "miette")]
mod miette {
    use crate::io::ReadError;
    use miette::{Diagnostic, LabeledSpan};

    impl Diagnostic for ReadError {
        fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
            let span = LabeledSpan::at(self.span.clone(), "here");

            Some(Box::new(std::iter::once(span)))
        }
    }
}
