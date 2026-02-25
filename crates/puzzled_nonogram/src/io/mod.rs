mod text;

#[cfg(feature = "puz")]
mod puz;

#[cfg(feature = "image")]
pub mod image;

use puzzled_io as io;
use std::path::Path;

use crate::Nonogram;

/// Unified puzzle loader.
pub fn read_puzzle_from_path<R>(path: R) -> Result<Nonogram, io::ReadError>
where
    R: AsRef<Path>,
{
    let path = path.as_ref();
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        #[cfg(feature = "puz")]
        "puz" => {
            use puzzled_io::PuzReader;

            let reader = PuzReader::default();
            let (nonogram, _) = reader.read_from_path(path)?;
            Ok(nonogram)
        }

        #[cfg(feature = "image")]
        "png" | "jpg" | "jpeg" => {
            use puzzled_io::ImageReader;

            let reader = ImageReader;
            let (nonogram, _) = reader.read_from_path(path)?;
            Ok(nonogram)
        }
        _ => Err(io::ReadError::UnsupportedFormat {
            format: ext.clone(),
        }),
    }
}
