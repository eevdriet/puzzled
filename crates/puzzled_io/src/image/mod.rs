pub mod read;
pub mod write;

pub use read::ImageReader;
pub use write::{ImageSizeCheck, ImageWriter, check_image_size};

use image::{DynamicImage, RgbaImage};

pub trait ImagePuzzle: Sized {
    fn read_image(image: &DynamicImage, reader: &ImageReader) -> read::Result<Self>;

    fn write_image(&self) -> write::Result<RgbaImage>;
}
