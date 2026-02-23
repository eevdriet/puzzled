use puzzled_core::Grid;

use crate::{Context, format, image::write};

pub trait ImageSizeCheck {
    fn check_image_size(&self) -> write::Result<()>;
}

impl<T> ImageSizeCheck for Grid<T> {
    fn check_image_size(&self) -> write::Result<()> {
        let max_size = u32::MAX as usize;

        check_image_size("Grid rows", self.rows(), max_size)?;
        check_image_size("Grid columns", self.cols(), max_size)?;
        Ok(())
    }
}

pub fn check_image_size<K>(kind: K, size: usize, max_size: usize) -> write::Result<()>
where
    K: Into<String>,
{
    if size > max_size {
        return Err(format::Error::SizeOverflow {
            kind: kind.into(),
            size,
            max_size,
        })
        .context("writing image");
    }

    Ok(())
}
