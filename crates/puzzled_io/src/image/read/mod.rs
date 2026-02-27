mod error;

pub use error::*;
use puzzled_core::Grid;

use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageReader as BaseImageReader, Rgba};

use crate::image::{ImagePuzzle, read};

#[derive(Debug, Default)]
pub struct ImageReader;

impl ImageReader {
    pub fn read<P, S>(&self, image: &DynamicImage) -> read::Result<(P, S)>
    where
        P: ImagePuzzle<S>,
    {
        let puzzle = P::read_image(image, self)?;

        Ok(puzzle)
    }

    pub fn read_from_path<R, P, S>(&self, path: R) -> read::Result<(P, S)>
    where
        R: AsRef<Path>,
        P: ImagePuzzle<S>,
    {
        let image = BaseImageReader::open(path)?.decode()?;
        self.read(&image)
    }

    pub fn read_grid<T, F>(&self, image: &DynamicImage, pixel_fn: &mut F) -> read::Result<Grid<T>>
    where
        F: FnMut(Rgba<u8>) -> read::Result<T>,
    {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let size = width.checked_mul(height).unwrap();

        let mut values = Vec::with_capacity(size);

        for (_, _, pixel) in image.pixels() {
            let value = pixel_fn(pixel)?;
            values.push(value);
        }

        let grid = Grid::from_vec(values, width).expect("Image represents valid grid");

        Ok(grid)
    }
}
