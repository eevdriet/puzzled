use image::{DynamicImage, Pixel, Rgba};
use puzzled_core::{Cell, Color, Metadata};
use puzzled_io::{
    Context, ImagePuzzle, ImageReader, format,
    image::{
        read,
        write::{self, write_grid_image},
    },
};

use crate::{Colors, Fill, Nonogram, NonogramState};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Puzzle does not define a color for {fill:?}")]
    MissingFillColor { fill: Fill },
}

impl ImagePuzzle<NonogramState> for Nonogram {
    fn width(&self) -> usize {
        self.fills().cols()
    }

    fn height(&self) -> usize {
        self.fills().rows()
    }

    fn read_image(
        image: &DynamicImage,
        reader: &ImageReader,
    ) -> read::Result<(Self, NonogramState)> {
        let mut colors = Colors::default();

        let mut read_pixel = |rgba: Rgba<u8>| {
            let [r, g, b, a] = rgba.0;
            let color = Color::rgba(r, g, b, a);

            let entry = match rgba.to_rgb().0 {
                // Fully white pixels are considered blank
                [255, 255, 255] => None,

                // Fully black pixels are considered crosses
                [0, 0, 0] => Some(Fill::Cross),

                _ => {
                    let idx = colors
                        .values()
                        .position(|&col| col == color)
                        .unwrap_or(colors.len()) as u32;
                    let fill = Fill::Color(idx);

                    colors.insert(fill, color);
                    Some(fill)
                }
            };

            Ok(Cell::new(entry))
        };

        let fills = reader.read_grid(image, &mut read_pixel)?;
        let metadata = Metadata::default();

        let nonogram = Nonogram::new(fills, colors, metadata).map_err(|err| {
            let puzzle_err = format::Error::PuzzleSpecific(Box::new(err));
            read::Error::Format(puzzle_err)
        })?;

        let state = NonogramState::from(&nonogram);
        Ok((nonogram, state))
    }

    fn write_image(&self, state: &NonogramState) -> write::Result<image::RgbaImage> {
        let colors = self.colors();

        let mut write_fill = |fill: &Option<Fill>| {
            let Some(fill) = fill else {
                return Ok(Rgba([0, 0, 0, 0]));
            };

            match colors.get(fill) {
                Some(color) => Ok(Rgba([color.red, color.green, color.blue, color.alpha])),
                None => {
                    let err = Error::MissingFillColor { fill: *fill };

                    Err(format::Error::PuzzleSpecific(Box::new(err))).context("Writing fill")
                }
            }
        };

        write_grid_image(state.solutions(), &mut write_fill)
    }
}
