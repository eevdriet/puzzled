use std::fmt;
use std::ops;

use crate::Axis;
use crate::LinePosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub col: u16,

    pub row: u16,
}

impl Position {
    pub const ORIGIN: Self = Self::new(0, 0);

    pub const fn new(row: u16, col: u16) -> Self {
        Self { row, col }
    }

    pub fn along_axis(&self, axis: Axis) -> LinePosition {
        match axis {
            Axis::Row => LinePosition::row_offset(self.row, self.col),
            Axis::Col => LinePosition::col_offset(self.col, self.row),
        }
    }

    pub fn relative(&self) -> (LinePosition, LinePosition) {
        (self.along_axis(Axis::Row), self.along_axis(Axis::Col))
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::ORIGIN
    }
}

impl From<(u16, u16)> for Position {
    fn from((row, col): (u16, u16)) -> Self {
        Self::new(row, col)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl ops::Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Self::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl ops::AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Position> for Position {
    type Output = Self;

    fn sub(self, rhs: Position) -> Self::Output {
        let row = self.row.saturating_sub(rhs.row);
        let col = self.col.saturating_sub(rhs.col);

        Self::new(row, col)
    }
}

impl ops::SubAssign<Position> for Position {
    fn sub_assign(&mut self, rhs: Position) {
        *self = *self - rhs;
    }
}
