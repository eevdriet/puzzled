use std::fmt;

use derive_more::{Deref, DerefMut};
use puzzled_core::{Cell, Reveal};

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

impl Reveal for CrosswordCell {
    fn reveal(&mut self) {
        self.0.reveal();
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

#[cfg(feature = "serde")]
mod serde_impl {
    use puzzled_core::Cell;
    use serde::{Deserialize, Serialize};

    use crate::{CrosswordCell, Solution};

    impl Serialize for CrosswordCell {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for CrosswordCell {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let cell = Cell::<Solution>::deserialize(deserializer)?;
            Ok(CrosswordCell::new(cell))
        }
    }
}
