use std::path::Path;

use image::{Rgba, RgbaImage};

use crate::{Nonogram, img};

#[derive(Debug, Default)]
pub struct ImageWriter;

impl ImageWriter {
    pub fn write(&self, puzzle: &Nonogram) -> img::Result<RgbaImage> {
        let fills = puzzle.fills();
        let rows = fills.rows() as u32;
        let cols = fills.cols() as u32;
        let max_size = u32::MAX;

        if rows > max_size || cols > max_size {
            return Err(img::Error::SizeOverflow { rows, cols });
        }

        let mut img = RgbaImage::new(cols, rows);
        let colors = puzzle.colors();

        for (pos, &fill) in fills.iter_indexed() {
            let color = colors
                .get(&fill)
                .ok_or(img::Error::MissingFillColor { fill })?;

            let pixel = Rgba([color.red, color.green, color.blue, color.alpha]);
            let x = pos.col as u32;
            let y = pos.row as u32;

            img.put_pixel(x, y, pixel);
        }

        Ok(img)
    }

    pub fn write_to_path<P>(&self, puzzle: &Nonogram, path: P) -> img::Result<()>
    where
        P: AsRef<Path>,
    {
        let img = self.write(puzzle)?;
        img.save(path)?;

        Ok(())
    }
}
