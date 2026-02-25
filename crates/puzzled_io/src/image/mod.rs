pub mod read;
pub mod write;

use puzzled_core::Puzzle;
pub use read::ImageReader;
pub use write::{ImageSizeCheck, ImageWriter, check_image_size};

use image::{DynamicImage, RgbaImage};

pub trait ImagePuzzle<S>: Puzzle {
    fn read_image(image: &DynamicImage, reader: &ImageReader) -> read::Result<(Self, S)>;

    fn write_image(&self, state: &S) -> write::Result<RgbaImage>;
}
