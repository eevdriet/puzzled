use std::path::Path;

use crate::{Colors, Fill, Fills, Nonogram, Rules, img};
use image::{DynamicImage, GenericImageView, ImageReader as BaseImageReader, Pixel};
use puzzled_core::{Color, Grid, Metadata};

#[derive(Debug, Default)]
pub struct ImageReader;

impl ImageReader {
    pub fn read(&self, image: &DynamicImage) -> img::Result<Nonogram> {
        let (fills, colors) = read_fills_and_colors(image)?;
        let rules = Rules::from_fills(&fills);

        let meta = Metadata::default();

        let puzzle = Nonogram::new(fills, rules, colors, meta);
        Ok(puzzle)
    }

    pub fn read_from_path<P>(&self, path: P) -> img::Result<Nonogram>
    where
        P: AsRef<Path>,
    {
        let image = BaseImageReader::open(path)?.decode()?;
        self.read(&image)
    }
}

fn read_fills_and_colors(image: &DynamicImage) -> img::Result<(Fills, Colors)> {
    let mut colors = Colors::default();

    let data: Vec<_> = image
        .pixels()
        .map(|(_, _, rgba)| {
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
        })
        .collect();

    let cols = image.width() as usize;
    let grid = Grid::from_vec(data, cols)?;
    let fills = Fills::new(grid);

    Ok((fills, colors))
}
