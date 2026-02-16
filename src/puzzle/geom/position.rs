use std::fmt;
use std::ops;

use crate::Offset;

/// 2-dimensional coordinate to be used within a [grid](crate::Grid)
///
/// The [`Position`] is commonly used in a [puzzle](crate::Puzzle) to access a specific [square](crate::Square).
/// An [`Offset`] can be used to easily create a new position from an existing one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    /// Row (y) coordinate
    pub row: u8,

    /// Column (x) coordinate
    pub col: u8,
}

impl Position {
    /// Origin position at (0, 0)
    pub const ORIGIN: Self = Self { row: 0, col: 0 };

    /// Construct a new position
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::ORIGIN
    }
}

impl From<(u8, u8)> for Position {
    fn from((row, col): (u8, u8)) -> Self {
        Position::new(row, col)
    }
}

impl From<Position> for (u8, u8) {
    fn from(pos: Position) -> Self {
        (pos.row, pos.col)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl ops::Add<Offset> for Position {
    type Output = Self;

    /// Moves the position by the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn add(self, offset: Offset) -> Self {
        let max = i16::from(u8::MAX);

        let col = i16::from(self.col)
            .saturating_add(offset.cols)
            .clamp(0, max) as u8;
        let row = i16::from(self.row)
            .saturating_add(offset.rows)
            .clamp(0, max) as u8;

        Self { col, row }
    }
}

impl ops::Sub<Offset> for Position {
    type Output = Self;

    /// Moves the position by the inverse of the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn sub(self, offset: Offset) -> Self {
        let max = i16::from(u8::MAX);

        let col = i16::from(self.col)
            .saturating_sub(offset.cols)
            .clamp(0, max) as u8;
        let row = i16::from(self.row)
            .saturating_sub(offset.rows)
            .clamp(0, max) as u8;

        Self { col, row }
    }
}

impl ops::AddAssign<Offset> for Position {
    /// Moves the position in place by the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn add_assign(&mut self, offset: Offset) {
        *self = *self + offset;
    }
}

impl ops::SubAssign<Offset> for Position {
    /// Moves the position in place by the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn sub_assign(&mut self, offset: Offset) {
        *self = *self - offset;
    }
}
