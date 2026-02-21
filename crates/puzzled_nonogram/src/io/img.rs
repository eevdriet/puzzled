use std::path::Path;

use crate::{Color, Fill, Fills, Nonogram, Rule, Rules, io};
use image::{DynamicImage, GenericImageView, ImageReader, Pixel, Rgba};
use puzzled_core::Grid;

pub struct ImageLoader;

impl io::PuzzleLoader for ImageLoader {
    fn load_nonogram(path: &Path) -> io::Result<Nonogram> {
        let image = ImageReader::open(path)?.decode()?;
        let (cols, rows) = image.dimensions();

        let grid = Grid::new(rows as usize, cols as usize).expect("Non-overflowing size");
        let fills = Fills::new(grid);
        let (rules, colors) = read_rules_and_colors(&image)?;

        Ok(Nonogram::new(fills, rules, colors))
    }
}

fn read_rules_and_colors(image: &DynamicImage) -> io::Result<(Rules, Vec<Color>)> {
    let mut colors = Vec::new();

    let mut pixel_to_fill = |pixel: Rgba<u8>| -> Fill {
        let [r, g, b] = pixel.to_rgb().0;
        let color = (r, g, b);

        match color {
            // Ignore fully filled/empty pixels
            (0, 0, 0) | (255, 255, 255) => Fill::Blank,

            _ => {
                let idx = match colors.iter().position(|&col| col == color) {
                    Some(idx) => idx + 1,
                    None => {
                        colors.push(color);
                        colors.len()
                    }
                };

                Fill::Color(idx)
            }
        }
    };

    let rows = iter_rows(image)
        .map(|row| {
            let fills = row.map(&mut pixel_to_fill);

            Rule::from_fills(fills)
        })
        .collect();

    let cols = iter_cols(image)
        .map(|row| {
            let fills = row.map(&mut pixel_to_fill);

            Rule::from_fills(fills)
        })
        .collect();

    Ok((Rules::new(rows, cols), colors))
}

fn iter_rows(
    img: &DynamicImage,
) -> impl Iterator<Item = impl Iterator<Item = image::Rgba<u8>> + '_> {
    let (width, height) = img.dimensions();

    (0..height).map(move |y| (0..width).map(move |x| img.get_pixel(x, y)))
}

fn iter_cols(
    img: &DynamicImage,
) -> impl Iterator<Item = impl Iterator<Item = image::Rgba<u8>> + '_> {
    let (width, height) = img.dimensions();

    (0..width).map(move |x| (0..height).map(move |y| img.get_pixel(x, y)))
}
