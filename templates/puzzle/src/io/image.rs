use image::{DynamicImage, Pixel, Rgba, RgbaImage};
use puzzled_io::{
    ImagePuzzle, ImageReader, format,
    image::{
        read,
        write::{self, ImageSizeCheck},
    },
};

impl ImagePuzzle<{{ puzzle | pascal_case}}State> for {{puzzle | pascal_case}} {
    fn read_image(
        image: &DynamicImage,
        reader: &ImageReader,
    ) -> read::Result<(Self, {{puzzle | pascal_case}}State)> {
        let {{ puzzle }} = {{ puzzle | pascal_case }}::new();
        let state = {{ puzzle | pascal_case }}State::from(&nonogram);

        Ok(({{ puzzle }}, state))
    }

    fn write_image(&self, state: &{{puzzle | pascal_case}}State) -> write::Result<image::RgbaImage> {
        let mut img = RgbaImage::new(cols, rows);
        Ok(img)
    }
}
