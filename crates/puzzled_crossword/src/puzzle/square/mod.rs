mod solution;
mod squares;

use puzzled_core::{Cell, Square};
pub use {solution::*, squares::*};

pub type CrosswordSquare = Square<Cell<Solution>>;
