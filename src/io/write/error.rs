use std::io;

use thiserror::Error;

use crate::io::Context;

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
    #[error("{0}")]
    Custom(String),

    #[error("I/O error: {0}")]
    Io(std::io::Error),

    #[error("Formatting error: {0}")]
    Format(crate::io::Error),
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
