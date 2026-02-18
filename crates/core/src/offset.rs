use crate::Position;

/// Amounts by which to move a [`Position`](crate::Position).
///
/// Positive numbers move to the right/bottom and negative to the left/top.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Offset {
    /// How many columns to move over by
    pub cols: i16,

    /// How many rows to move over by
    pub rows: i16,
}

impl Offset {
    /// A zero offset
    pub const ZERO: Self = Self { cols: 0, rows: 0 };
    /// A left offset
    pub const LEFT: Self = Self { cols: -1, rows: 0 };
    /// A right offset
    pub const RIGHT: Self = Self { cols: 1, rows: 0 };
    /// An up offset
    pub const UP: Self = Self { cols: 0, rows: -1 };
    /// A down offset
    pub const DOWN: Self = Self { cols: 0, rows: 1 };
}

impl From<Position> for Offset {
    fn from(position: Position) -> Self {
        Self {
            cols: i16::from(position.col),
            rows: i16::from(position.row),
        }
    }
}
