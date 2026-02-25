mod cells;

pub use cells::*;

use puzzled_core::Cell;

use crate::Fill;

pub type NonogramCell = Cell<Fill>;
