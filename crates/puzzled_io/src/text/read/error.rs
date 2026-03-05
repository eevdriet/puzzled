use std::fmt::Debug;

use chumsky::{error::Rich, span::SimpleSpan};

use crate::format;

pub type Span = SimpleSpan<usize>;
pub type ParseError<'a> = Rich<'a, char>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Found parsing errors: {0:?}")]
    Parse(Vec<String>),

    #[error("Found invalid metadata property \"{found}\": {reason}")]
    InvalidMetaProperty { found: String, reason: String },

    #[error("{format}")]
    Format { format: format::Error, span: Span },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl Error {
    pub fn format(format: format::Error, span: Span) -> Self {
        Self::Format { format, span }
    }
}
