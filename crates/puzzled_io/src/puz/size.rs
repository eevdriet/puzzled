use puzzled_core::Grid;

use crate::{Context, format, puz::write};

pub trait PuzSizeCheck {
    const KIND: &'static str;

    fn check_puz_size(&self) -> write::Result<()>;
}

impl<T> PuzSizeCheck for Grid<T> {
    const KIND: &'static str = "Grid";

    fn check_puz_size(&self) -> write::Result<()> {
        let max_size = u8::MAX as usize;

        check_puz_size("Grid rows", self.rows(), max_size)?;
        check_puz_size("Grid columns", self.cols(), max_size)?;
        Ok(())
    }
}

pub fn check_puz_size<K>(kind: K, size: usize, max_size: usize) -> write::Result<()>
where
    K: Into<String>,
{
    if size > max_size {
        return Err(format::Error::SizeOverflow {
            kind: kind.into(),
            size,
            max_size,
        })
        .context("writing .puz");
    }

    Ok(())
}
