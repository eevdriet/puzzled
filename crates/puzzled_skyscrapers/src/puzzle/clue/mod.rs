mod clues;

pub use clues::*;
use puzzled_core::Line;

use crate::Direction;

#[derive(Debug, PartialEq, Eq)]
pub struct ClueId {
    line: Line,
    direction: Direction,
}

impl ClueId {
    pub fn new(line: Line, direction: Direction) -> Self {
        Self { line, direction }
    }
}
