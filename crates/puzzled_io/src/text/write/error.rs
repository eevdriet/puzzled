#[derive(Debug, thiserror::Error)]
#[error("Write error: {0}")]
pub enum Error {
    #[error("Write error")]
    Custom,
}

pub type Result<T> = core::result::Result<T, Error>;
