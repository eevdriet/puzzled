pub mod read;
pub mod write;

pub use read::{ImageReader, read_grid};
pub use write::{ImageSizeCheck, ImageWriter, check_image_size};

mod error;
pub use error::*;

use image::{DynamicImage, RgbaImage};

pub trait ImagePuzzle: Sized {
    fn from_image(image: &DynamicImage) -> read::Result<Self>;

    fn to_image(&self) -> write::Result<RgbaImage>;
}
