use std::{fmt, ops};

use crate::{Clue, ClueDirection, Crossword};

/// Type that identifies where a [clue](Clue) is placed within a [puzzle](Crossword)
///
/// The identifier mimics the way clues are commonly identified in real crosswords.
/// For example, "4 across" can be specified as `(4, Direction::Across)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClueId {
    pub num: u8,
    pub direction: ClueDirection,
}

impl fmt::Display for ClueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.num, self.direction)
    }
}

impl From<(u8, ClueDirection)> for ClueId {
    fn from((num, direction): (u8, ClueDirection)) -> Self {
        Self { num, direction }
    }
}

impl ops::Index<ClueId> for Crossword {
    type Output = Clue;

    /// Index the puzzle to retrieve a reference to the [clue](Clue) that was [identified](ClueId)
    /// ```
    /// use puzzled::crossword::{clue, crossword, Direction::*};
    ///
    /// let puzzle = crossword! (
    ///     [A B]
    ///     [C .]
    ///     - A: "AB"
    ///     - D: "AC"
    ///     - D: "B"
    ///     - A: "C"
    /// );
    ///
    /// assert_eq!(puzzle[(1, Across)], clue!(1 A: "AB" @ (0, 0) + 2));
    /// assert_eq!(puzzle[(1, Down)], clue!(1 D: "AC" @ (0, 0) + 2));
    /// assert_eq!(puzzle[(2, Down)], clue!(2 D: "B" @ (0, 1) + 1));
    /// assert_eq!(puzzle[(3, Across)], clue!(3 A: "C" @ (1, 0) + 1));
    /// ```
    ///
    /// # Panics
    /// Panics if the given `pos` is out of bounds, i.e. `pos.row >= puzzle.rows() || pos.col >= puzzle.cols()`.
    /// ```should_panic
    /// use puzzled::crossword::{crossword, Direction::*};
    ///
    /// let puzzle = crossword! (
    ///    [A B]
    ///    [C D]
    /// );
    ///
    /// let clue = &puzzle[(10, Across)];
    /// ```
    fn index(&self, id: ClueId) -> &Self::Output {
        &self.clues[&id]
    }
}
