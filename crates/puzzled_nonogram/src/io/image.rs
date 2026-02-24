use image::{DynamicImage, Pixel, Rgba, RgbaImage};
use puzzled_core::{Cell, Color, Metadata};
use puzzled_io::{
    ImagePuzzle, ImageReader, format,
    image::{
        read,
        write::{self, ImageSizeCheck},
    },
};

use crate::{Colors, Fill, Fills, Nonogram, NonogramCell};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Puzzle does not define a color for {fill:?}")]
    MissingFillColor { fill: Fill },
}

impl ImagePuzzle for Nonogram {
    fn read_image(image: &DynamicImage, reader: &ImageReader) -> read::Result<Self> {
        let mut colors = Colors::default();

        let mut read_pixel = |rgba: Rgba<u8>| {
            let [r, g, b, a] = rgba.0;
            let color = Color::rgba(r, g, b, a);

            let fill = match rgba.to_rgb().0 {
                // Ignore fully filled/empty pixels
                [0, 0, 0] | [255, 255, 255] => Fill::Blank,

                _ => {
                    let idx = colors
                        .values()
                        .position(|&col| col == color)
                        .unwrap_or(colors.len()) as u32;
                    let fill = Fill::Color(idx);

                    colors.insert(fill, color);
                    fill
                }
            };

            let cell = Cell::new(fill);
            NonogramCell::new(cell)
        };

        let fills = reader.read_grid(image, &mut read_pixel)?;
        let fills = Fills::new(fills);
        let metadata = Metadata::default();

        Ok(Nonogram::new(fills, colors, metadata))
    }

    fn write_image(&self) -> write::Result<image::RgbaImage> {
        let fills = self.fills();
        fills.check_image_size()?;
        let rows = fills.rows() as u32;
        let cols = fills.cols() as u32;

        let mut img = RgbaImage::new(cols, rows);
        let colors = self.colors();

        for (pos, cell) in fills.iter_indexed() {
            let fill = *cell.solution();
            let color = colors.get(&fill).ok_or_else(|| {
                let err = Error::MissingFillColor { fill };

                format::Error::PuzzleSpecific(Box::new(err))
            })?;

            let pixel = Rgba([color.red, color.green, color.blue, color.alpha]);
            let x = pos.col as u32;
            let y = pos.row as u32;

            img.put_pixel(x, y, pixel);
        }

        Ok(img)
    }
}
