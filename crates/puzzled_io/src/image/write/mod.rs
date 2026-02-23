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
    pub fn write<P>(&self, puzzle: &P) -> write::Result<RgbaImage>
    where
        P: ImagePuzzle,
    {
        let image = puzzle.write_image()?;
        Ok(image)
    }

    pub fn write_to_path<R, P>(&self, puzzle: &P, path: R) -> write::Result<()>
    where
        R: AsRef<Path>,
        P: ImagePuzzle,
    {
        let img = self.write(puzzle)?;
        img.save(path)?;

        Ok(())
    }
}
