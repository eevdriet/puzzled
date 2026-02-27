mod error;
mod size;

pub use error::*;
use puzzled_core::Grid;
pub use size::*;

use std::path::Path;

use crate::image::{ImagePuzzle, write};
use image::{Rgba, RgbaImage};

#[derive(Debug, Default)]
pub struct ImageWriter;

impl ImageWriter {
    pub fn write<P, S>(&self, puzzle: &P, state: &S) -> write::Result<RgbaImage>
    where
        P: ImagePuzzle<S>,
    {
        // Verify that the puzzle is sized correctly
        let width = puzzle.width();
        let height = puzzle.height();

        check_image_size("Puzzle width", width, u32::MAX as usize)?;
        check_image_size("Puzzle height", height, u32::MAX as usize)?;

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

pub fn write_grid_image<T, F>(solutions: &Grid<T>, mut pixel_fn: F) -> write::Result<RgbaImage>
where
    F: FnMut(&T) -> write::Result<Rgba<u8>>,
{
    let rows = solutions.rows() as u32;
    let cols = solutions.cols() as u32;

    let mut img = RgbaImage::new(cols, rows);

    for (pos, solution) in solutions.iter_indexed() {
        let pixel = pixel_fn(solution)?;
        let x = pos.col as u32;
        let y = pos.row as u32;

        img.put_pixel(x, y, pixel);
    }

    Ok(img)
}
