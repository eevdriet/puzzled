use std::io;

use thiserror::Error;

use crate::io::{Context, format};

#[derive(Debug, Error)]
#[error("{kind} while writing '{context}'")]
pub struct Error {
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
}

impl<T> Context<T, Error> for io::Result<T> {
    fn context<S: Into<String>>(self, context: S) -> Result<T> {
        self.map_err(|err| Error {
            kind: ErrorKind::Io(err),
            context: context.into(),
        })
    }
}

pub type Result<T> = core::result::Result<T, Error>;
