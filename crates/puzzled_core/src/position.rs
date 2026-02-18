use std::fmt;
use std::ops;

use crate::Offset;
use crate::Order;
use crate::clamped_add;

/// 2-dimensional coordinate to be used within a [grid](crate::Grid)
///
/// An [`Offset`] can be used to easily create a new position from an existing one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    /// Row (y) coordinate
    pub row: usize,

    /// Column (x) coordinate
    pub col: usize,
}

impl Position {
    /// Origin position at (0, 0)
    pub const ORIGIN: Self = Self { row: 0, col: 0 };

    /// Construct a new position
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn from_row_order(idx: usize, cols: usize) -> Self {
        Self {
            row: idx / cols,
            col: idx % cols,
        }
    }

    pub fn from_col_order(idx: usize, rows: usize) -> Self {
        Self {
            row: idx % rows,
            col: idx / rows,
        }
    }

    pub fn offset(&self, offset: Offset) -> Self {
        let row = clamped_add(self.row, offset.rows);
        let col = clamped_add(self.col, offset.cols);

        Self { col, row }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::ORIGIN
    }
}

impl From<(usize, usize)> for Position {
    fn from((row, col): (usize, usize)) -> Self {
        Position::new(row, col)
    }
}

impl From<Position> for (usize, usize) {
    fn from(pos: Position) -> Self {
        (pos.row, pos.col)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "R{}, C{}", self.row, self.col)
    }
}

impl ops::Add<Offset> for Position {
    type Output = Self;

    /// Moves the position by the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn add(self, offset: Offset) -> Self {
        self.offset(offset)
    }
}

impl ops::Sub<Offset> for Position {
    type Output = Self;

    /// Moves the position by the inverse of the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn sub(self, offset: Offset) -> Self {
        self.offset(-offset)
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
