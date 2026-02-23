// Text format
pub mod text;

#[doc(inline)]
pub use text::{TxtPuzzle, TxtReader};

pub mod format;

// Puz format
#[cfg(feature = "puz")]
pub mod puz;

#[cfg(feature = "puz")]
#[doc(inline)]
pub use puz::{BinaryPuzzle, PuzReader, PuzWriter};

// Image format
#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "image")]
#[doc(inline)]
pub use image::{ImagePuzzle, ImageReader, ImageWriter};

// Other
mod error;
pub use error::*;
