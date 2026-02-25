mod error;
mod size;

pub use error::*;
pub use size::*;

use std::path::Path;

use crate::image::{ImagePuzzle, write};
use image::RgbaImage;

#[derive(Debug, Default)]
pub struct ImageWriter;

impl ImageWriter {
    pub fn write<P, S>(&self, puzzle: &P, state: &S) -> write::Result<RgbaImage>
    where
        P: ImagePuzzle<S>,
    {
        let image = puzzle.write_image(state)?;
        Ok(image)
    }

    pub fn write_to_path<R, P, S>(&self, puzzle: &P, state: &S, path: R) -> write::Result<()>
    where
        R: AsRef<Path>,
        P: ImagePuzzle<S>,
    {
        let img = self.write(puzzle, state)?;
        img.save(path)?;

        Ok(())
    }
}
