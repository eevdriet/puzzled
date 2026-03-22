use std::ops::Bound;

use crate::{Direction, Grid, Line, LineSegment, Offset, Order, Position};

pub struct GridLinearIter<'a, T> {
    pub(crate) grid: &'a Grid<T>,
    pub(crate) front: Position,
    pub(crate) back: Position,
    pub(crate) offset: Offset,
    pub(crate) remaining: usize,
}

impl<'a, T> GridLinearIter<'a, T> {
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

    pub fn new_single(grid: &'a Grid<T>, single: Position) -> Self {
        Self {
            grid,
            front: single,
            back: single,
            offset: Offset::default(),
            remaining: 1,
        }
    }

    pub fn new_with_remaining(
        grid: &'a Grid<T>,
        start: Position,
        offset: Offset,
        remaining: usize,
    ) -> Self {
        let end = match start.offset(offset * remaining.saturating_sub(1) as isize) {
            Some(end) => end,
            _ => start,
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

impl<'a, T> Iterator for GridLinearIter<'a, T> {
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

impl<'a, T> DoubleEndedIterator for GridLinearIter<'a, T> {
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

impl<'a, T> ExactSizeIterator for GridLinearIter<'a, T> {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<'a, T> Clone for GridLinearIter<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for GridLinearIter<'a, T> {}

#[doc(hidden)]
pub struct GridLinearIterMut<'a, T> {
    pub(crate) grid: &'a mut Grid<T>,
    pub(crate) front: Position,
    pub(crate) back: Position,
    pub(crate) offset: Offset,
    pub(crate) remaining: usize,
}

impl<'a, T> GridLinearIterMut<'a, T> {
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

impl<'a, T> Iterator for GridLinearIterMut<'a, T> {
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

impl<'a, T> DoubleEndedIterator for GridLinearIterMut<'a, T> {
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

impl<'a, T> ExactSizeIterator for GridLinearIterMut<'a, T> {
    fn len(&self) -> usize {
        self.remaining
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
    pub fn iter_row(&self, row: usize) -> GridLinearIter<'_, T> {
        GridLinearIter::new_row(self, row)
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
    pub fn iter_row_mut(&mut self, row: usize) -> GridLinearIterMut<'_, T> {
        GridLinearIterMut::new_row(self, row)
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
    pub fn iter_rows(&self) -> impl Iterator<Item = GridLinearIter<'_, T>> {
        (0..self.rows).map(move |row| GridLinearIter::new_row(self, row))
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
    pub fn iter_col(&self, col: usize) -> GridLinearIter<'_, T> {
        GridLinearIter::new_col(self, col)
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
    pub fn iter_col_mut(&mut self, col: usize) -> GridLinearIterMut<'_, T> {
        GridLinearIterMut::new_col(self, col)
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
    pub fn iter_cols(&self) -> impl Iterator<Item = GridLinearIter<'_, T>> {
        (0..self.cols).map(move |col| GridLinearIter::new_col(self, col))
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
    pub fn iter_line<'a>(&'a self, line: Line) -> GridLinearIter<'a, T> {
        match line {
            Line::Row(row) => GridLinearIter::new_row(self, row),
            Line::Col(col) => GridLinearIter::new_col(self, col),
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
    pub fn iter_line_mut<'a>(&'a mut self, line: Line) -> GridLinearIterMut<'a, T> {
        match line {
            Line::Row(row) => GridLinearIterMut::new_row(self, row),
            Line::Col(col) => GridLinearIterMut::new_col(self, col),
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
    pub fn iter_lines<'a>(&'a self, order: Order) -> impl Iterator<Item = GridLinearIter<'a, T>> {
        let lines = match order {
            Order::Rows => 0..self.rows,
            Order::Cols => 0..self.cols,
        };

        lines.map(move |line| match order {
            Order::Rows => GridLinearIter::new_row(self, line),
            Order::Cols => GridLinearIter::new_col(self, line),
        })
    }
}

impl<T> Grid<T> {
    pub fn iter_segment(&self, pos: Position, dir: Direction) -> GridLinearIter<'_, T> {
        let segment = LineSegment::from((pos, dir));

        match segment.line {
            Line::Row(row) if row < self.rows => {
                match resolve_range(&segment.start, &segment.end, self.cols) {
                    Some((start, end)) => {
                        GridLinearIter::new_with_remaining(self, pos, dir.into(), end - start)
                    }
                    _ => GridLinearIter::new_empty(self),
                }
            }
            Line::Col(col) if col < self.cols => {
                match resolve_range(&segment.start, &segment.end, self.rows) {
                    Some((start, end)) => {
                        GridLinearIter::new_with_remaining(self, pos, dir.into(), end - start)
                    }
                    _ => GridLinearIter::new_empty(self),
                }
            }
            _ => GridLinearIter::new_empty(self),
        }
    }

    pub fn iter_segment_mut(&mut self, pos: Position, dir: Direction) -> GridLinearIterMut<'_, T> {
        let segment = LineSegment::from((pos, dir));

        match segment.line {
            Line::Row(row) if row < self.rows => {
                match resolve_range(&segment.start, &segment.end, self.cols) {
                    Some((start, end)) => {
                        GridLinearIterMut::new_with_remaining(self, pos, dir.into(), end - start)
                    }
                    _ => GridLinearIterMut::new_empty(self),
                }
            }
            Line::Col(col) if col < self.cols => {
                match resolve_range(&segment.start, &segment.end, self.rows) {
                    Some((start, end)) => {
                        GridLinearIterMut::new_with_remaining(self, pos, dir.into(), end - start)
                    }
                    _ => GridLinearIterMut::new_empty(self),
                }
            }
            _ => GridLinearIterMut::new_empty(self),
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

    (start <= end && end <= max).then_some((start, end))
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::resolve_range;
    use crate::{Direction, Grid, grid};
    use crate::{Direction::*, LineSegment};

    #[fixture]
    fn grid() -> Grid<usize> {
        grid!(
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            [11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
            [33, 32, 33, 34, 35, 36, 37, 38, 39, 40],
            [44, 42, 43, 44, 45, 46, 47, 48, 49, 50],
            [55, 52, 53, 54, 55, 56, 57, 58, 59, 60],
            [66, 62, 63, 64, 65, 66, 67, 68, 69, 70],
            [77, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            [88, 82, 83, 84, 85, 86, 87, 88, 89, 90],
            [99, 92, 93, 94, 95, 96, 97, 98, 99, 100],
        )
    }

    #[rstest]
    #[case(33, Right, vec![33, 34, 35, 36, 37, 38, 39, 40])]
    #[case(33, Up, vec![33, 23, 13, 3])]
    fn iter_segment(#[case] num: usize, #[case] dir: Direction, #[case] expected: Vec<usize>) {
        use crate::Line;

        let grid = grid();
        let pos = grid.position(num - 1).expect("Specified valid number");
        let segment = LineSegment::from((pos, dir));

        let range = match segment.line {
            Line::Row(row) if row < grid.rows => {
                resolve_range(&segment.start, &segment.end, grid.rows)
            }
            Line::Col(col) if col < grid.cols => {
                resolve_range(&segment.start, &segment.end, grid.rows)
            }
            _ => None,
        };
        eprintln!("Range: {range:?}");

        if range.is_none() {
            panic!(
                "Invalid segment: {segment:?} (with {} rows and {} cols)",
                grid.rows, grid.cols
            );
        }

        let nums: Vec<_> = grid.iter_segment(pos, dir).copied().collect();
        assert_eq!(expected, nums);
    }
}
