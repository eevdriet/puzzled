use crate::Nonogram;
use std::path::Path;

mod error;
mod img;
mod json;
mod text;

pub use error::*;

pub trait PuzzleLoader {
    fn load_nonogram(path: &Path) -> Result<Nonogram>;
}

pub fn load_nonogram(path: impl AsRef<Path>) -> Result<Nonogram> {
    let path = path.as_ref();
    let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    match ext {
        "json" => json::JsonLoader::load_nonogram(path),
        "png" | "jpg" | "jpeg" => img::ImageLoader::load_nonogram(path),
        "txt" | "text" => text::TextLoader::load_nonogram(path),
        _ => Err(Error::UnsupportedExtension(ext.to_string())),
    }
}
