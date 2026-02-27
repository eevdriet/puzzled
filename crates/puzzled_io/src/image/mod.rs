pub mod read;
pub mod write;

use std::{io, path::PathBuf};

use puzzled_core::Puzzle;
pub use read::ImageReader;
pub use write::{ImageWriter, check_image_size};

use image::{DynamicImage, RgbaImage};

use crate::puzzle_dir;

pub trait ImagePuzzle<S>: Puzzle {
    // Read the puzzle from an image
    fn read_image(image: &DynamicImage, reader: &ImageReader) -> read::Result<(Self, S)>;

    // Write the puzzle into an image
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn write_image(&self, state: &S) -> write::Result<RgbaImage>;

    fn load_image(&self, name: &str) -> crate::image::read::Result<(Self, S)> {
        let reader = crate::image::ImageReader;

        let dir = puzzle_dir::<Self>()?;
        let path = dir.join(name).with_extension("puz");

        reader.read_from_path(path)
    }

    fn save_image(&self, name: &str) -> crate::image::write::Result<()>
    where
        S: for<'a> From<&'a Self>,
    {
        let state = S::from(self);
        self.save_image_with_state(name, &state)
    }

    fn save_image_with_state(&self, name: &str, state: &S) -> crate::image::write::Result<()>
    where
        S: for<'a> From<&'a Self>,
    {
        let writer = crate::ImageWriter;
        writer.write_to_file(self, &state, name)
    }
}
