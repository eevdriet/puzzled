use std::io;

use puzzled_core::GridError;

use crate::Fill;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Grid error: {0}")]
    Grid(#[from] GridError),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    #[error(
        "Puzzle is too large to write to an image ({rows} rows and {cols}, max side length of {} allowed)",
        u32::MAX
    )]
    SizeOverflow { rows: u32, cols: u32 },

    #[error("Puzzle does not define a color for {fill:?}")]
    MissingFillColor { fill: Fill },
}

pub type Result<T> = std::result::Result<T, Error>;
