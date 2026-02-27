use image::{DynamicImage, Pixel, Rgba};
use puzzled_core::{Cell, Color, Entry, Metadata, Timer};
use puzzled_io::{
    Context, ImagePuzzle, ImageReader, format,
    image::{
        read,
        write::{self, write_grid_image},
    },
};

use crate::{Binario, BinarioState, Bit, BitError};

impl ImagePuzzle<BinarioState> for Binario {
    fn width(&self) -> usize {
        self.cells().cols()
    }

    fn height(&self) -> usize {
        self.cells().rows()
    }

    fn read_image(
        image: &DynamicImage,
        reader: &ImageReader,
    ) -> read::Result<(Self, BinarioState)> {
        let mut read_pixel = |rgba: Rgba<u8>| {
            let bit = match rgba.to_rgb().0 {
                [0, 0, 0] => Bit::Zero,
                [255, 255, 255] => Bit::One,

                [r, g, b] => {
                    let color = Color::rgb(r, g, b);
                    let err = Box::new(BitError::InvalidColor(color));

                    return Err(format::Error::PuzzleSpecific(err)).context("Reading bit");
                }
            };

            Ok(Cell::new(Some(bit)))
        };

        let cells = reader.read_grid(image, &mut read_pixel)?;
        let solutions = cells.map_ref(|cell| cell.solution);
        let entries = cells.map_ref(|cell| Entry::new_with_style(None, cell.style));

        let binario = Binario::new(cells, Metadata::default());

        let timer = Timer::default();
        let state = BinarioState::new(solutions, entries, timer);

        Ok((binario, state))
    }

    fn write_image(&self, state: &BinarioState) -> write::Result<image::RgbaImage> {
        let mut write_bill = |bit: &Option<Bit>| {
            let rgba = match bit {
                None => Rgba([255, 255, 255, 0]),
                Some(Bit::Zero) => Rgba([0, 0, 0, 255]),
                Some(Bit::One) => Rgba([0, 0, 0, 255]),
            };

            Ok(rgba)
        };

        write_grid_image(state.solutions(), &mut write_bill)
    }
}
