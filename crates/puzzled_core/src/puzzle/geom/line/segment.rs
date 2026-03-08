use std::ops::{Bound, RangeBounds};

use derive_more::Debug;

use crate::{Direction, Line, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSegment {
    pub line: Line,
    pub start: Bound<usize>,
    pub end: Bound<usize>,
}

impl LineSegment {
    pub fn new<R>(line: Line, segment: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        Self {
            line,
            start: segment.start_bound().cloned(),
            end: segment.end_bound().cloned(),
        }
    }

    pub fn with_line<R>(&self, line: Line) -> Self
    where
        R: RangeBounds<usize> + Clone,
    {
        Self {
            line,
            start: self.start,
            end: self.end,
        }
    }
}

impl From<(Position, Direction)> for LineSegment {
    fn from((pos, dir): (Position, Direction)) -> Self {
        match dir {
            Direction::Up => Self {
                line: Line::Col(pos.col),
                start: Bound::Unbounded,
                end: Bound::Included(pos.row),
            },
            Direction::Down => Self {
                line: Line::Col(pos.col),
                start: Bound::Included(pos.row),
                end: Bound::Unbounded,
            },
            Direction::Left => Self {
                line: Line::Row(pos.row),
                start: Bound::Unbounded,
                end: Bound::Included(pos.col),
            },
            Direction::Right => Self {
                line: Line::Row(pos.row),
                start: Bound::Included(pos.col),
                end: Bound::Unbounded,
            },
        }
    }
}
