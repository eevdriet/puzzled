use std::ops;

use crate::{Line, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinePosition {
    pub line: Line,
    pub offset: u16,
}

impl LinePosition {
    // Constructors
    pub fn new(line: Line, offset: u16) -> Self {
        Self { line, offset }
    }

    pub fn row_offset(row: u16, offset: u16) -> Self {
        Self {
            line: Line::Row(row),
            offset,
        }
    }

    pub fn col_offset(col: u16, offset: u16) -> Self {
        Self {
            line: Line::Col(col),
            offset,
        }
    }

    pub fn with_offset(&self, offset: u16) -> Self {
        Self {
            line: self.line,
            offset,
        }
    }

    pub fn absolute(&self) -> Position {
        match self.line {
            Line::Row(row) => Position {
                row,
                col: self.offset,
            },
            Line::Col(col) => Position {
                col,
                row: self.offset,
            },
        }
    }
}

impl From<LinePosition> for Position {
    fn from(pos: LinePosition) -> Self {
        match pos.line {
            Line::Row(row) => Position {
                row,
                col: pos.offset,
            },
            Line::Col(col) => Position {
                col,
                row: pos.offset,
            },
        }
    }
}

impl ops::Add<u16> for LinePosition {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self::new(self.line, self.offset + rhs)
    }
}

impl ops::AddAssign<u16> for LinePosition {
    fn add_assign(&mut self, rhs: u16) {
        *self = *self + rhs;
    }
}

impl ops::Sub<u16> for LinePosition {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self::Output {
        Self::new(self.line, self.offset.saturating_sub(rhs))
    }
}

impl ops::SubAssign<u16> for LinePosition {
    fn sub_assign(&mut self, rhs: u16) {
        *self = *self - rhs;
    }
}
