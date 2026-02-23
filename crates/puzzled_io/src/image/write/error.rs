#[derive(Debug, thiserror::Error)]
#[error("Write error: {0}")]
pub enum Error {
    #[error("Write error")]
    Custom,
}
