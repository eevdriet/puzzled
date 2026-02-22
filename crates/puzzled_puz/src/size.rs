use puzzled_core::Grid;

use crate::format;

pub trait SizeCheck {
    const KIND: &'static str;

    fn check_size(&self) -> format::Result<()>;
}

impl<T> SizeCheck for Grid<T> {
    const KIND: &'static str = "Grid";

    fn check_size(&self) -> format::Result<()> {
        let max_size = u8::MAX as usize;

        check_size("Grid rows", self.rows(), max_size)?;
        check_size("Grid columns", self.cols(), max_size)?;
        Ok(())
    }
}

pub fn check_size<K>(kind: K, size: usize, max_size: usize) -> format::Result<()>
where
    K: Into<String>,
{
    if size > max_size {
        return Err(format::Error::SizeOverflow {
            kind: kind.into(),
            size,
            max_size,
        });
    }

    Ok(())
}
