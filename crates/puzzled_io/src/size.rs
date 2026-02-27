use crate::{Context, format};

pub fn check_size<K>(kind: K, size: usize, max_size: usize) -> write::Result<()>
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
