use std::{fmt, ops};

use crate::{Line, Offset, Position, clamped_add};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinePosition {
    pub line: Line,
    pub pos: usize,
}

impl LinePosition {
    // Constructors
    pub fn new(line: Line, pos: usize) -> Self {
        Self { line, pos }
    }

    pub fn with_line(&self, line: Line) -> Self {
        Self {
            line,
            pos: self.pos,
        }
    }

    pub fn with_pos(&self, pos: usize) -> Self {
        Self {
            line: self.line,
            pos,
        }
    }

    pub fn offset(&self, offset: Offset) -> Self {
        let (line, pos) = match self.line {
            Line::Row(row) => {
                let pos = clamped_add(self.pos, offset.cols);
                let line = clamped_add(row, offset.rows);

                (Line::Row(line), pos)
            }
            Line::Col(col) => {
                let pos = clamped_add(self.pos, offset.rows);
                let line = clamped_add(col, offset.cols);

                (Line::Col(line), pos)
            }
        };

        Self { line, pos }
    }

    pub fn absolute(&self) -> Position {
        match self.line {
            Line::Row(row) => Position { row, col: self.pos },
            Line::Col(col) => Position { col, row: self.pos },
        }
    }
}

impl From<LinePosition> for Position {
    fn from(pos: LinePosition) -> Self {
        match pos.line {
            Line::Row(row) => Position { row, col: pos.pos },
            Line::Col(col) => Position { col, row: pos.pos },
        }
    }
}

impl fmt::Display for LinePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self.line {
            Line::Row(_) => 'C',
            Line::Col(_) => 'R',
        };

        write!(f, "{} @ {}{}", self.line, char, self.pos)
    }
}

impl ops::Add<Offset> for LinePosition {
    type Output = Self;

    fn add(self, offset: Offset) -> Self::Output {
        self.offset(offset)
    }
}

impl ops::AddAssign<Offset> for LinePosition {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Offset> for LinePosition {
    type Output = Self;

    fn sub(self, offset: Offset) -> Self::Output {
        self.offset(-offset)
    }
}

impl ops::SubAssign<Offset> for LinePosition {
    fn sub_assign(&mut self, offset: Offset) {
        *self = *self - offset;
    }
}

impl ops::Add<usize> for LinePosition {
    type Output = Self;

    fn add(self, offset: usize) -> Self::Output {
        let pos = self.pos.saturating_add(offset);
        self.with_pos(pos)
    }
}

impl ops::AddAssign<usize> for LinePosition {
    fn add_assign(&mut self, offset: usize) {
        *self = *self + offset;
    }
}

impl ops::Sub<usize> for LinePosition {
    type Output = Self;

    fn sub(self, offset: usize) -> Self::Output {
        let pos = self.pos.saturating_sub(offset);
        self.with_pos(pos)
    }
}

impl ops::SubAssign<usize> for LinePosition {
    fn sub_assign(&mut self, offset: usize) {
        *self = *self - offset;
    }
}
