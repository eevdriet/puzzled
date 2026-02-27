pub mod read;
pub mod write;

use puzzled_core::Puzzle;
pub use read::ImageReader;
pub use write::{ImageWriter, check_image_size};

use image::{DynamicImage, RgbaImage};

pub trait ImagePuzzle<S>: Puzzle {
    // Read the puzzle from an image
    fn read_image(image: &DynamicImage, reader: &ImageReader) -> read::Result<(Self, S)>;

    // Write the puzzle into an image
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn write_image(&self, state: &S) -> write::Result<RgbaImage>;
}
