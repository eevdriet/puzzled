// Text format
pub mod text;
pub use text::{TxtPuzzle, TxtReader};

pub mod format;

// Puz format
#[cfg(feature = "puz")]
pub mod puz;

#[cfg(feature = "puz")]
pub use puz::{Puz, PuzReader, PuzWriter};

// Image format
#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "image")]
pub use image::{ImagePuzzle, ImageReader, ImageWriter};

// Other
mod error;
pub use error::*;
