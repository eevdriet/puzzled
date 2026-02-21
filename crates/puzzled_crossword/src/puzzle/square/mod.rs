mod cell;
mod solution;
mod squares;
mod style;

pub use {cell::*, solution::*, squares::*, style::*};
pub type Square = Option<Cell>;

pub(crate) const EMPTY_SQUARE: char = '.';
