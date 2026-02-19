use puzzled_core::Position;

use crate::{Clue, Direction};

/// Specification for how to add a [clue](Clue) to a [puzzle](Crossword).
///
/// This struct can be used when the user is unsure what [squares](crate::Square) the clue should correspond to.
/// By calling [`Crossword::place_clues`], the specs are turned into [clues](Clue) by placing them from the next available square in the puzzle.
/// Furthermore, [`Crossword::insert_clues`] can be used to add the clues to the puzzle after positioning them.
///
/// The [`clue_spec!`](crate::clue_spec) macro provides a shorthand for creating clue specifications.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClueSpec {
    text: String,
    direction: Direction,
}

impl ClueSpec {
    /// Specify a clue from its [direction](Direction) and text
    pub fn new<S: Into<String>>(direction: Direction, text: S) -> Self {
        Self {
            direction,
            text: text.into(),
        }
    }

    /// Specify a [across](Direction::Across) clue
    pub fn across<S: Into<String>>(text: S) -> Self {
        Self::new(Direction::Across, text.into())
    }

    /// Specify a [down](Direction::Down) clue
    pub fn down<S: Into<String>>(text: S) -> Self {
        Self::new(Direction::Down, text.into())
    }

    /// Clue text
    pub fn text(&self) -> &String {
        &self.text
    }

    /// [Direction] in which the clue should be placed in a [puzzle](Crossword)
    pub fn direction(&self) -> Direction {
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
