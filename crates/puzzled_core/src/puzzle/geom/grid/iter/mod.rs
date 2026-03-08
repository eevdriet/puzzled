mod indexed;

use std::ops::Bound;

pub use indexed::*;

use crate::{Grid, Line, LineSegment, Offset, Order, Position};

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Iter<'a, T> {
    grid: &'a Grid<T>,
    front: Position,
    back: Position,
    offset: Offset,
    remaining: usize,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(grid: &'a Grid<T>, start: Position, offset: Offset) -> Self {
        let (remaining, end) = if !grid.is_in_bounds(start) {
            (0, start)
        } else if offset == Offset::ZERO {
            (1, start)
        } else {
            let row_steps = steps_to_edge(start.row, offset.rows, grid.rows);
            let col_steps = steps_to_edge(start.col, offset.cols, grid.cols);
            let steps = row_steps.min(col_steps);

            let remaining = steps + 1; // include the starting position
            let back = start
                .offset(offset * steps as isize)
                .expect("Calculated remaining steps");

            (remaining, back)
        };

        Self {
            grid,
            offset,
            front: start,
            back: end,
            remaining,
        }
    }

    pub fn new_empty(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            front: Position::default(),
            back: Position::default(),
            offset: Offset::default(),
            remaining: 0,
        }
    }

    pub fn new_with_remaining(
        grid: &'a Grid<T>,
        start: Position,
        offset: Offset,
        remaining: usize,
    ) -> Self {
        let end = if remaining == 0 {
            start
        } else {
            start
                .offset(offset * (remaining - 1) as isize)
                .unwrap_or_else(|| panic!("End position should be consistent with {remaining} items from (and including) {start}"))
        };

        Self {
            grid,
            offset,
            front: start,
            back: end,
            remaining,
        }
    }

    pub fn new_row(grid: &'a Grid<T>, row: usize) -> Self {
        Self {
            grid,
            offset: Offset::RIGHT,
            front: Position::new(row, 0),
            back: Position::new(row, grid.cols() - 1),
            remaining: grid.cols(),
        }
    }

    pub fn new_col(grid: &'a Grid<T>, col: usize) -> Self {
        Self {
            grid,
            offset: Offset::DOWN,
            front: Position::new(0, col),
            back: Position::new(grid.rows() - 1, col),
            remaining: grid.rows(),
        }
    }
}

fn steps_to_edge(start: usize, step: isize, max: usize) -> usize {
    if step > 0 {
        (max - 1 - start) / step as usize
    } else if step < 0 {
        start / (-step) as usize
    } else {
        usize::MAX
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let pos = self.front;
        self.front = self.front.offset(self.offset).unwrap_or(self.front);
        self.remaining -= 1;

        self.grid.get(pos)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let pos = self.back;
        self.back = self.back.offset(-self.offset).unwrap_or(self.back);
        self.remaining -= 1;

        self.grid.get(pos)
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.remaining
    }
}

#[doc(hidden)]
pub struct IterMut<'a, T> {
    grid: &'a mut Grid<T>,
    front: Position,
    back: Position,
    offset: Offset,
    remaining: usize,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(grid: &'a mut Grid<T>, start: Position, offset: Offset) -> Self {
        let (remaining, end) = if !grid.is_in_bounds(start) {
            (0, start)
        } else {
            let row_steps = steps_to_edge(start.row, offset.rows, grid.rows);
            let col_steps = steps_to_edge(start.col, offset.cols, grid.cols);
            let steps = row_steps.min(col_steps);

            let remaining = steps + 1; // include the starting position
            let back = start
                .offset(offset * steps as isize)
                .expect("Calculated remaining steps");

            (remaining, back)
        };

        Self {
            grid,
            offset,
            front: start,
            back: end,
            remaining,
        }
    }

    pub fn new_row(grid: &'a mut Grid<T>, row: usize) -> Self {
        let cols = grid.cols();

        Self {
            grid,
            offset: Offset::RIGHT,
            front: Position::new(row, 0),
            back: Position::new(row, cols - 1),
            remaining: cols,
        }
    }

    pub fn new_col(grid: &'a mut Grid<T>, col: usize) -> Self {
        let rows = grid.rows();

        Self {
            grid,
            offset: Offset::DOWN,
            front: Position::new(0, col),
            back: Position::new(rows - 1, col),
            remaining: rows,
        }
    }

    pub fn new_with_remaining(
        grid: &'a mut Grid<T>,
        start: Position,
        offset: Offset,
        remaining: usize,
    ) -> Self {
        let end = if remaining == 0 {
            start
        } else {
            start
                .offset(offset * (remaining - 1) as isize)
                .unwrap_or_else(|| panic!("End position should be consistent with {remaining} items from (and including) {start}"))
        };

        Self {
            grid,
            offset,
            front: start,
            back: end,
            remaining,
        }
    }

    pub fn new_empty(grid: &'a mut Grid<T>) -> Self {
        Self {
            grid,
            front: Position::default(),
            back: Position::default(),
            offset: Offset::default(),
            remaining: 0,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let pos = self.front;
        self.front = self.front.offset(self.offset).unwrap_or(self.front);
        self.remaining -= 1;

        // SAFETY: we should never yield overlapping positions
        unsafe { Some(&mut *(&mut self.grid[pos] as *mut T)) }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let pos = self.back;
        self.back = self.back.offset(-self.offset).unwrap_or(self.back);
        self.remaining -= 1;

        // SAFETY: we should never yield overlapping positions
        unsafe { Some(&mut *(&mut self.grid[pos] as *mut T)) }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<T> Grid<T> {
    /// Creates an iterator over the [positions](Position) of the grid
    ///
    /// The entries are traversed in [row-major](crate::Order::Rows) order.
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.positions();
    /// assert_eq!(iter.next(), Some(Position::new(0, 0)));
    /// assert_eq!(iter.next(), Some(Position::new(0, 1)));
    /// assert_eq!(iter.next(), Some(Position::new(1, 0)));
    /// assert_eq!(iter.next(), Some(Position::new(1, 1)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn positions(&self) -> impl Iterator<Item = Position> {
        (0..self.data.len()).map(move |idx| self.position(idx).expect("Position should be valid"))
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.data.into_iter()
    }

    /// Creates an iterator over the grid
    ///
    /// The entries are traversed in [row-major](crate::Order::Rows) order.
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter();
    /// assert_eq!(iter.next(), Some(&'A'));
    /// assert_eq!(iter.next(), Some(&'B'));
    /// assert_eq!(iter.next(), Some(&'C'));
    /// assert_eq!(iter.next(), Some(&'D'));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Creates a mutable iterator over the grid
    ///
    /// The entries are traversed in [row-major](crate::Order::Rows) order.
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let mut grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_mut();
    /// assert_eq!(iter.next(), Some(&mut 'A'));
    /// assert_eq!(iter.next(), Some(&mut 'B'));
    /// assert_eq!(iter.next(), Some(&mut 'C'));
    /// assert_eq!(iter.next(), Some(&mut 'D'));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }
}

impl<T> Grid<T> {
    /// Creates an iterator over a specified row of the grid
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_row(0);
    /// assert_eq!(iter.next(), Some(&'A'));
    /// assert_eq!(iter.next(), Some(&'B'));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_row(1);
    /// assert_eq!(iter.next(), Some(&'C'));
    /// assert_eq!(iter.next(), Some(&'D'));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_row(2);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_row(&self, row: usize) -> Iter<'_, T> {
        Iter::new_row(self, row)
    }

    /// Creates a mutable iterator over a specified row of the grid
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let mut grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_row_mut(0);
    /// assert_eq!(iter.next(), Some(&mut 'A'));
    /// assert_eq!(iter.next(), Some(&mut 'B'));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_row_mut(1);
    /// assert_eq!(iter.next(), Some(&mut 'C'));
    /// assert_eq!(iter.next(), Some(&mut 'D'));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_row_mut(2);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_row_mut(&mut self, row: usize) -> IterMut<'_, T> {
        IterMut::new_row(self, row)
    }

    /// Creates an iterator over the rows of the grid
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let grid = grid![
    ///    [1, 2, 3],
    ///    [4, 5, 6],
    ///    [7, 8, 9]
    /// ];
    /// let row_sums: Vec<usize> = grid.iter_rows().map(|row| row.sum()).collect();
    /// assert_eq!(row_sums, vec![6, 15, 24]);
    /// ```
    pub fn iter_rows(&self) -> impl Iterator<Item = Iter<'_, T>> {
        (0..self.rows).map(move |row| Iter::new_row(self, row))
    }

    /// Creates an iterator over a specified column of the grid
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_col(0);
    /// assert_eq!(iter.next(), Some(&'A'));
    /// assert_eq!(iter.next(), Some(&'C'));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_col(1);
    /// assert_eq!(iter.next(), Some(&'B'));
    /// assert_eq!(iter.next(), Some(&'D'));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_col(2);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_col(&self, col: usize) -> Iter<'_, T> {
        Iter::new_col(self, col)
    }

    /// Creates a mutable iterator over a specified column of the grid
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let mut grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_col_mut(0);
    /// assert_eq!(iter.next(), Some(&mut 'A'));
    /// assert_eq!(iter.next(), Some(&mut 'C'));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_col_mut(1);
    /// assert_eq!(iter.next(), Some(&mut 'B'));
    /// assert_eq!(iter.next(), Some(&mut 'D'));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_col_mut(2);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_col_mut(&mut self, col: usize) -> IterMut<'_, T> {
        IterMut::new_col(self, col)
    }

    /// Creates an iterator over the columns of the grid
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let grid = grid![
    ///    [1, 2, 3],
    ///    [4, 5, 6],
    ///    [7, 8, 9]
    /// ];
    /// let col_sums: Vec<usize> = grid.iter_cols().map(|col| col.sum()).collect();
    /// assert_eq!(col_sums, vec![12, 15, 18]);
    /// ```
    pub fn iter_cols(&self) -> impl Iterator<Item = Iter<'_, T>> {
        (0..self.cols).map(move |col| Iter::new_col(self, col))
    }

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
    pub fn iter_line<'a>(&'a self, line: Line) -> Iter<'a, T> {
        match line {
            Line::Row(row) => Iter::new_row(self, row),
            Line::Col(col) => Iter::new_col(self, col),
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
    pub fn iter_line_mut<'a>(&'a mut self, line: Line) -> IterMut<'a, T> {
        match line {
            Line::Row(row) => IterMut::new_row(self, row),
            Line::Col(col) => IterMut::new_col(self, col),
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
    pub fn iter_lines<'a>(&'a self, order: Order) -> impl Iterator<Item = Iter<'a, T>> {
        let lines = match order {
            Order::Rows => 0..self.rows,
            Order::Cols => 0..self.cols,
        };

        lines.map(move |line| match order {
            Order::Rows => Iter::new_row(self, line),
            Order::Cols => Iter::new_col(self, line),
        })
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

    (start <= end && start < max && end < max).then_some((start, end))
}

impl<T> Grid<T> {
    pub fn iter_segment(&self, segment: &LineSegment) -> Iter<'_, T> {
        match segment.line {
            Line::Row(row) if row < self.rows => {
                match resolve_range(&segment.start, &segment.end, self.cols) {
                    Some((start, end)) => {
                        let pos = Position::new(row, start);
                        Iter::new_with_remaining(self, pos, Offset::RIGHT, end - start)
                    }
                    _ => Iter::new_empty(self),
                }
            }
            Line::Col(col) if col < self.cols => {
                match resolve_range(&segment.start, &segment.end, self.rows) {
                    Some((start, end)) => {
                        let pos = Position::new(start, col);
                        Iter::new_with_remaining(self, pos, Offset::DOWN, end - start)
                    }
                    _ => Iter::new_empty(self),
                }
            }
            _ => Iter::new_empty(self),
        }
    }

    pub fn iter_segment_mut(&mut self, segment: &LineSegment) -> IterMut<'_, T> {
        match segment.line {
            Line::Row(row) if row < self.rows => {
                match resolve_range(&segment.start, &segment.end, self.cols) {
                    Some((start, end)) => {
                        let pos = Position::new(row, start);
                        IterMut::new_with_remaining(self, pos, Offset::RIGHT, end - start)
                    }
                    _ => IterMut::new_empty(self),
                }
            }
            Line::Col(col) if col < self.cols => {
                match resolve_range(&segment.start, &segment.end, self.rows) {
                    Some((start, end)) => {
                        let pos = Position::new(start, col);
                        IterMut::new_with_remaining(self, pos, Offset::DOWN, end - start)
                    }
                    _ => IterMut::new_empty(self),
                }
            }
            _ => IterMut::new_empty(self),
        }
    }
}
