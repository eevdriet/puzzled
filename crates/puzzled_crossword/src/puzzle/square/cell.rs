use std::fmt;

use derive_more::{Deref, DerefMut};
use puzzled_core::Cell;

use crate::Solution;

#[derive(Deref, DerefMut, PartialEq, Eq, Clone)]
pub struct CrosswordCell(Cell<Solution>);

impl CrosswordCell {
    pub fn new(cell: Cell<Solution>) -> Self {
        Self(cell)
    }

    /// Construct a filled square that has a [one-letter solution](Solution::Letter)
    pub fn letter(letter: char) -> Self {
        Self(Cell::new(Solution::Letter(letter)))
    }

    /// Construct a filled square that has a [rebus solution](Solution::Rebus)
    pub fn rebus(rebus: String) -> Self {
        Self(Cell::new(Solution::Rebus(rebus)))
    }

    /// Verify whether the solution to the cell is a letter
    pub fn is_letter(&self) -> bool {
        matches!(self.solution(), &Solution::Letter(_))
    }

    /// Verify whether the solution to the cell is a rebus
    pub fn is_rebus(&self) -> bool {
        matches!(self.solution(), &Solution::Rebus(_))
    }
}

impl fmt::Display for CrosswordCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for CrosswordCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
