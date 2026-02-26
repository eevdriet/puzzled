mod cells;
mod direction;

pub use cells::*;
pub use direction::*;

use puzzled_core::Cell;

pub type SkyscraperCell = Cell<u8>;
