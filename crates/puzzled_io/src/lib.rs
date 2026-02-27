// Text format
#[cfg(feature = "text")]
pub mod text;

use puzzled_core::{Cell, Entry, Grid, Square};
#[cfg(feature = "text")]
#[doc(inline)]
pub use text::{TxtPuzzle, TxtReader};

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
mod util;

pub use error::*;
pub use util::*;

pub mod format;

pub(crate) type CellEntries<T> = (Grid<Cell<T>>, Grid<Entry<T>>);
pub(crate) type SquareEntries<T> = (Grid<Square<Cell<T>>>, Grid<Square<Entry<T>>>);
