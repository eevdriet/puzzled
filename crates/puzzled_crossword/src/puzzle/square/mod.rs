mod cell;
mod solution;
mod squares;

pub use {cell::*, solution::*, squares::*};
pub type Square = Option<CrosswordCell>;

pub(crate) const EMPTY_SQUARE: char = '.';
