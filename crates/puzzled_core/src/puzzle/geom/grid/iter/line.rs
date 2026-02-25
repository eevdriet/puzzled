use std::{
    iter::{Skip, Take},
    ops::{Bound, RangeBounds},
};

use crate::{ColIter, ColIterMut, Grid, Line, LineSegment, Order, RowIter, RowIterMut};

#[doc(hidden)]
#[derive(Clone)]
pub enum LineIter<'a, T> {
    Row(RowIter<'a, T>),
    Col(ColIter<'a, T>),
    Empty,
}

impl<'a, T> Iterator for LineIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Row(iter) => iter.next(),
            Self::Col(iter) => iter.next(),
            _ => None,
        }
    }
}

impl<'a, T> DoubleEndedIterator for LineIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Row(iter) => iter.next_back(),
            Self::Col(iter) => iter.next_back(),
            _ => None,
        }
    }
}

impl<'a, T> ExactSizeIterator for LineIter<'a, T> {
    fn len(&self) -> usize {
        match self {
            Self::Row(iter) => iter.len(),
            Self::Col(iter) => iter.len(),
            Self::Empty => 0,
        }
    }
}

#[doc(hidden)]
pub enum LineIterMut<'a, T> {
    Row(RowIterMut<'a, T>),
    Col(ColIterMut<'a, T>),
    Empty,
}

impl<'a, T> Iterator for LineIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Row(iter) => iter.next(),
            Self::Col(iter) => iter.next(),
            _ => None,
        }
    }
}

impl<'a, T> DoubleEndedIterator for LineIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Row(iter) => iter.next_back(),
            Self::Col(iter) => iter.next_back(),
            _ => None,
        }
    }
}

impl<'a, T> ExactSizeIterator for LineIterMut<'a, T> {
    fn len(&self) -> usize {
        match self {
            Self::Row(iter) => iter.len(),
            Self::Col(iter) => iter.len(),
            Self::Empty => 0,
        }
    }
}

impl<T> Grid<T> {
    /// Creates an iterator over a specified [line](Line) of the grid
    /// ```
    /// use puzzled_core::{grid, Line};
    ///
    /// let grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_line(Line::Row(0));
    /// assert_eq!(iter.next(), Some(&'A'));
    /// assert_eq!(iter.next(), Some(&'B'));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_line(Line::Col(0));
    /// assert_eq!(iter.next(), Some(&'A'));
    /// assert_eq!(iter.next(), Some(&'C'));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_line<'a>(&'a self, line: Line) -> LineIter<'a, T> {
        match line {
            Line::Row(row) if row < self.rows => {
                let iter = self.iter_row(row);
                LineIter::Row(iter)
            }
            Line::Col(col) if col < self.cols => {
                let iter = self.iter_col(col);
                LineIter::Col(iter)
            }
            _ => LineIter::Empty,
        }
    }

    /// Creates a mutable iterator over a specified [line](Line) of the grid
    /// ```
    /// use puzzled_core::{grid, Line};
    ///
    /// let mut grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    ///
    /// {
    ///     let mut iter = grid.iter_line_mut(Line::Row(0));
    ///     assert_eq!(iter.next(), Some(&mut 'A'));
    ///     assert_eq!(iter.next(), Some(&mut 'B'));
    ///     assert_eq!(iter.next(), None);
    /// }
    /// {
    ///     let mut iter = grid.iter_line_mut(Line::Col(0));
    ///     assert_eq!(iter.next(), Some(&mut 'A'));
    ///     assert_eq!(iter.next(), Some(&mut 'C'));
    ///     assert_eq!(iter.next(), None);
    /// }
    /// ```
    pub fn iter_line_mut<'a>(&'a mut self, line: Line) -> LineIterMut<'a, T> {
        match line {
            Line::Row(row) if row < self.rows => {
                let iter = self.iter_row_mut(row);
                LineIterMut::Row(iter)
            }
            Line::Col(col) if col < self.cols => {
                let iter = self.iter_col_mut(col);
                LineIterMut::Col(iter)
            }
            _ => LineIterMut::Empty,
        }
    }

    /// Creates an iterator over the [lines](Line) of the grid in the specified [order](Order)
    /// ```
    /// use puzzled_core::{grid, Order};
    ///
    /// let grid = grid![
    ///    [1, 2, 3],
    ///    [4, 5, 6],
    ///    [7, 8, 9]
    /// ];
    /// let row_sums: Vec<usize> = grid.iter_lines(Order::Rows).map(|row| row.sum()).collect();
    /// assert_eq!(row_sums, vec![6, 15, 24]);
    ///
    /// let col_sums: Vec<usize> = grid.iter_lines(Order::Cols).map(|col| col.sum()).collect();
    /// assert_eq!(col_sums, vec![12, 15, 18]);
    /// ```
    pub fn iter_lines<'a>(&'a self, order: Order) -> impl Iterator<Item = LineIter<'a, T>> {
        let lines = match order {
            Order::Rows => 0..self.rows,
            Order::Cols => 0..self.cols,
        };

        lines.map(move |line| match order {
            Order::Rows => {
                let iter = self.iter_row(line);
                LineIter::Row(iter)
            }
            Order::Cols => {
                let iter = self.iter_col(line);
                LineIter::Col(iter)
            }
        })
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub enum LineSegmentIter<'a, T> {
    Row(Take<Skip<RowIter<'a, T>>>),
    Col(Take<Skip<ColIter<'a, T>>>),
    Empty,
}

impl<'a, T> Iterator for LineSegmentIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Row(iter) => iter.next(),
            Self::Col(iter) => iter.next(),
            Self::Empty => None,
        }
    }
}

impl<'a, T> DoubleEndedIterator for LineSegmentIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Row(iter) => iter.next_back(),
            Self::Col(iter) => iter.next_back(),
            Self::Empty => None,
        }
    }
}

impl<'a, T> ExactSizeIterator for LineSegmentIter<'a, T> {
    fn len(&self) -> usize {
        match self {
            Self::Row(iter) => iter.len(),
            Self::Col(iter) => iter.len(),
            Self::Empty => 0,
        }
    }
}

fn resolve_range(start: &Bound<usize>, end: &Bound<usize>, max: usize) -> Option<(usize, usize)> {
    let start = match &start {
        Bound::Included(start) => *start,
        Bound::Excluded(start) => *start + 1,
        Bound::Unbounded => 0,
    };

    let end = match &end {
        Bound::Included(end) => *end + 1,
        Bound::Excluded(end) => *end,
        Bound::Unbounded => max,
    };

    (start < max && end < max).then_some((start, end))
}

impl<T> Grid<T> {
    pub fn iter_segment<'a>(&'a self, segment: &LineSegment) -> LineSegmentIter<'a, T> {
        match segment.line {
            Line::Row(row) if row < self.rows => {
                let iter = self.iter_row(row);

                match resolve_range(&segment.start, &segment.end, self.cols) {
                    Some((start, end)) => LineSegmentIter::Row(iter.skip(start).take(end - start)),
                    _ => LineSegmentIter::Empty,
                }
            }
            Line::Col(col) if col < self.cols => {
                let iter = self.iter_col(col);

                match resolve_range(&segment.start, &segment.end, self.rows) {
                    Some((start, end)) => LineSegmentIter::Col(iter.skip(start).take(end - start)),
                    _ => LineSegmentIter::Empty,
                }
            }
            _ => LineSegmentIter::Empty,
        }
    }
}
