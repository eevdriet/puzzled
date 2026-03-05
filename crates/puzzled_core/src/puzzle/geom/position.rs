use std::fmt;
use std::ops;

use crate::Direction;
use crate::Line;
use crate::LinePosition;
use crate::LineSegment;
use crate::Offset;
use crate::Order;

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

    pub fn offset(&self, offset: Offset) -> Option<Self> {
        let row = (self.row as isize).checked_add(offset.rows)?;
        let col = (self.col as isize).checked_add(offset.cols)?;

        if row < 0 || col < 0 {
            return None;
        }

        Some(Self {
            row: row as usize,
            col: col as usize,
        })
    }

    pub fn lines(&self) -> (Line, Line) {
        (Line::Row(self.row), Line::Col(self.col))
    }

    pub fn relative(&self) -> (LinePosition, LinePosition) {
        let (row, col) = self.lines();

        (
            LinePosition::new(row, self.col),
            LinePosition::new(col, self.row),
        )
    }

    pub fn with_order(&self, order: Order) -> LinePosition {
        match order {
            Order::Rows => LinePosition::new(Line::Row(self.row), self.col),
            Order::Cols => LinePosition::new(Line::Col(self.col), self.row),
        }
    }

    pub fn as_segment(&self, direction: Direction) -> LineSegment {
        match direction {
            Direction::Up => LineSegment::new(Line::Col(self.row), ..self.row + 1),
            Direction::Down => LineSegment::new(Line::Col(self.row), self.row..),
            Direction::Left => LineSegment::new(Line::Row(self.col), ..self.col + 1),
            Direction::Right => LineSegment::new(Line::Row(self.col), ..self.row),
        }
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
        write!(f, "{},{}", self.row + 1, self.col + 1)
    }
}

impl ops::Add<Offset> for Position {
    type Output = Option<Self>;

    /// Moves the position by the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn add(self, offset: Offset) -> Option<Self> {
        self.offset(offset)
    }
}

impl ops::Sub<Offset> for Position {
    type Output = Option<Self>;

    /// Moves the position by the inverse of the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn sub(self, offset: Offset) -> Option<Self> {
        self.offset(-offset)
    }
}

impl ops::AddAssign<Offset> for Position {
    /// Moves the position in place by the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn add_assign(&mut self, offset: Offset) {
        if let Some(pos) = *self + offset {
            *self = pos;
        }
    }
}

impl ops::SubAssign<Offset> for Position {
    /// Moves the position in place by the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn sub_assign(&mut self, offset: Offset) {
        if let Some(pos) = *self - offset {
            *self = pos;
        }
    }
}

#[cfg(feature = "serde")]
mod serde {
    use serde::{Deserialize, Serialize};

    use crate::Position;

    type PositionData = [usize; 2];

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Position {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let data: PositionData = [self.row, self.col];
            data.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Position {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let [row, col] = PositionData::deserialize(deserializer)?;
            Ok(Position { row, col })
        }
    }
}
