use puzzled_core::Position;

use crate::{Clue, ClueDirection};

/// Specification for how to add a [clue](Clue) to a [crossword](crate::Crossword).
///
/// This struct can be used when the user is unsure what [squares](crate::Square) the clue should correspond to.
/// By calling [`Crossword::place_clues`](crate::Crossword::place_clues), the specs are turned into [clues](Clue) by placing them from the next available square in the puzzle.
/// Furthermore, [`Crossword::insert_clues`](crate::Crossword::insert_clues) can be used to add the clues to the puzzle after positioning them.
///
/// The [`clue_spec!`](crate::clue_spec) macro provides a shorthand for creating clue specifications.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClueSpec {
    text: String,
    direction: ClueDirection,
}

impl ClueSpec {
    /// Specify a clue from its [direction](ClueDirection) and text
    pub fn new<S: Into<String>>(direction: ClueDirection, text: S) -> Self {
        Self {
            direction,
            text: text.into(),
        }
    }

    /// Specify a [across](ClueDirection::Across) clue
    pub fn across<S: Into<String>>(text: S) -> Self {
        Self::new(ClueDirection::Across, text.into())
    }

    /// Specify a [down](ClueDirection::Down) clue
    pub fn down<S: Into<String>>(text: S) -> Self {
        Self::new(ClueDirection::Down, text.into())
    }

    /// Clue text
    pub fn text(&self) -> &String {
        &self.text
    }

    /// [ClueDirection] in which the clue should be placed in a [crossword](crate::Crossword)
    pub fn direction(&self) -> ClueDirection {
        self.direction
    }

    /// Construct a [clue](Clue) from its specification and placement
    pub fn place(self, num: u8, start: Position, len: u8) -> Clue {
        Clue {
            num,
            start,
            len,
            text: self.text,
            direction: self.direction,
        }
    }
}
