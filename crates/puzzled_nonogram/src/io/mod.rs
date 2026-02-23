mod text;

use puzzled_io as io;
use std::path::Path;

use crate::Nonogram;

#[cfg(feature = "puz")]
mod puz;

#[cfg(feature = "image")]
pub mod img;

/// Unified puzzle loader.
pub fn read_puzzle_from_path<R>(path: R) -> io::Result<Nonogram>
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
            use puzzled_io::puz::PuzReader;

            let reader = PuzReader::default();
            Ok(reader.read_from_path(path).map_err(io::puz::Error::Read)?)
        }

        // #[cfg(feature = "image")]
        // "png" | "jpg" | "jpeg" => {
        //     use img::ImageReader;
        //
        //     let reader = ImageReader;
        //     Ok(reader.read_from_path(path)?)
        // }
        _ => Err(io::Error::UnsupportedExtension(ext.clone())),
    }
}
