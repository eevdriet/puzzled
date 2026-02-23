use image::{DynamicImage, Pixel, Rgba, RgbaImage};
use puzzled_core::{Color, Metadata};
use puzzled_io::{
    ImagePuzzle, format,
    image::{
        read, read_grid,
        write::{self, ImageSizeCheck},
    },
};

use crate::{Colors, Fill, Fills, Nonogram, Rules};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Puzzle does not define a color for {fill:?}")]
    MissingFillColor { fill: Fill },
}

impl ImagePuzzle for Nonogram {
    fn from_image(image: &DynamicImage) -> read::Result<Self> {
        let mut colors = Colors::default();

        let mut read_pixel = |rgba: Rgba<u8>| {
            let [r, g, b, a] = rgba.0;
            let color = Color::rgba(r, g, b, a);

            match rgba.to_rgb().0 {
                // Ignore fully filled/empty pixels
                [0, 0, 0] | [255, 255, 255] => Fill::Blank,

                _ => {
                    let idx = colors
                        .values()
                        .position(|&col| col == color)
                        .unwrap_or(colors.len());
                    let fill = Fill::Color(idx);

                    colors.insert(fill, color);
                    fill
                }
            }
        };

        let fills = read_grid(image, &mut read_pixel)?;
        let fills = Fills::new(fills);

        let rules = Rules::from_fills(&fills);
        let metadata = Metadata::default();

        Ok(Nonogram::new(fills, rules, colors, metadata))
    }

    fn to_image(&self) -> write::Result<image::RgbaImage> {
        let fills = self.fills();
        fills.check_image_size()?;
        let rows = fills.rows() as u32;
        let cols = fills.cols() as u32;

        let mut img = RgbaImage::new(cols, rows);
        let colors = self.colors();

        for (pos, &fill) in fills.iter_indexed() {
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
