mod error;
mod text;

use std::path::Path;

pub use error::*;

use crate::Nonogram;

#[cfg(feature = "puz")]
mod puz;

#[cfg(feature = "image")]
pub mod img;

/// Unified puzzle loader.
pub fn read_puzzle_from_path<P>(path: P) -> Result<Nonogram>
where
    P: AsRef<Path>,
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
            use puzzled_puz::PuzReader;

            let reader = PuzReader::default();
            Ok(reader.read_from_path(path)?)
        }

        #[cfg(feature = "image")]
        "png" | "jpg" | "jpeg" => {
            use img::ImageReader;

            let reader = ImageReader;
            Ok(reader.read_from_path(path)?)
        }

        _ => Err(Error::UnsupportedExtension(ext.clone())),
    }
}
