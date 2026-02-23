#[derive(Debug, thiserror::Error)]
#[error("Read error: {0}")]
pub enum Error {
    #[error("Read error")]
    Custom,
}
