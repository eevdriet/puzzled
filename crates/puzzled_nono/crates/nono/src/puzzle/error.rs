use thiserror::Error;

#[derive(Debug, Error)]
pub enum PuzzleError {
    #[error("{rows}x{cols} puzzle has {size} cells (should have {})", rows * cols)]
    SizeMismatch { rows: u16, cols: u16, size: usize },
}
