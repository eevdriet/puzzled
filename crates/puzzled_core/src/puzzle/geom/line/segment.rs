use std::ops::{Bound, RangeBounds};

use derive_more::Debug;

use crate::Line;

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
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}
