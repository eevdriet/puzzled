use puzzled_core::Line;

use crate::Direction;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClueId {
    pub direction: Direction,
    pub line: Line,
}

impl ClueId {
    pub fn new(line: Line, direction: Direction) -> Self {
        Self { line, direction }
    }
}

impl From<(Line, Direction)> for ClueId {
    fn from((line, direction): (Line, Direction)) -> Self {
        Self { line, direction }
    }
}

impl From<ClueId> for (Line, Direction) {
    fn from(id: ClueId) -> Self {
        (id.line, id.direction)
    }
}
