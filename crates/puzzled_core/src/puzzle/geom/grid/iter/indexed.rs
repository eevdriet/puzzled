use crate::{
    Direction, Grid, GridIter, GridLinearIter, GridLinearIterMut, GridPositionsIter, Line, Order,
    Position,
};

#[derive(Debug, Clone)]
pub struct GridIndexedIter<'a, T> {
    inner: GridIter<'a, T>,
}

impl<'a, T> GridIndexedIter<'a, T> {
    pub fn new(iter: GridIter<'a, T>) -> Self {
        Self { inner: iter }
    }
    pub fn new_linear(iter: GridLinearIter<'a, T>) -> Self {
        Self {
            inner: GridIter::Linear(iter),
        }
    }
    pub fn new_positions(iter: GridPositionsIter<'a, T>) -> Self {
        Self {
            inner: GridIter::Positions(iter),
        }
    }
}

impl<'a, T> Iterator for GridIndexedIter<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.len() == 0 {
            return None;
        }

        let pos = match &self.inner {
            GridIter::Linear(iter) => iter.front,
            GridIter::Positions(iter) => iter.positions[iter.front],
        };
        let item = self.inner.next()?;

        Some((pos, item))
    }
}

impl<'a, T> DoubleEndedIterator for GridIndexedIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.inner.len() == 0 {
            return None;
        }

        let pos = match &self.inner {
            GridIter::Linear(iter) => iter.back,
            GridIter::Positions(iter) => iter.positions[iter.back.checked_sub(1)?],
        };
        let item = self.inner.next_back()?;

        Some((pos, item))
    }
}

impl<'a, T> ExactSizeIterator for GridIndexedIter<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct GridIndexedIterMut<'a, T> {
    inner: GridLinearIterMut<'a, T>,
}

impl<'a, T> GridIndexedIterMut<'a, T> {
    pub fn new(iter: GridLinearIterMut<'a, T>) -> Self {
        Self { inner: iter }
    }
}

impl<'a, T> Iterator for GridIndexedIterMut<'a, T> {
    type Item = (Position, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.remaining == 0 {
            return None;
        }

        let pos = self.inner.front;
        let item = self.inner.next()?;

        Some((pos, item))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for GridIndexedIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.inner.remaining == 0 {
            return None;
        }

        let pos = self.inner.front;
        let item = self.inner.next_back()?;

        Some((pos, item))
    }
}

impl<'a, T> ExactSizeIterator for GridIndexedIterMut<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Grid<T> {
    /// Creates an indexed iterator over the grid
    ///
    /// The entries are traversed in [row-major](crate::Order::Rows) order.
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_indexed();
    /// assert_eq!(iter.next(), Some((Position::new(0, 0), &'A')));
    /// assert_eq!(iter.next(), Some((Position::new(0, 1), &'B')));
    /// assert_eq!(iter.next(), Some((Position::new(1, 0), &'C')));
    /// assert_eq!(iter.next(), Some((Position::new(1, 1), &'D')));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_indexed(&self) -> impl Iterator<Item = (Position, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, item)| (self.position(idx).expect("Only taking valid indices"), item))
    }

    /// Creates a mutable indexed iterator over the grid
    ///
    /// The entries are traversed in [row-major](crate::Order::Rows) order.
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let mut grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_indexed_mut();
    /// assert_eq!(iter.next(), Some((Position::new(0, 0), &mut 'A')));
    /// assert_eq!(iter.next(), Some((Position::new(0, 1), &mut 'B')));
    /// assert_eq!(iter.next(), Some((Position::new(1, 0), &mut 'C')));
    /// assert_eq!(iter.next(), Some((Position::new(1, 1), &mut 'D')));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_indexed_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> {
        self.data.iter_mut().enumerate().map(|(idx, item)| {
            let row = idx / self.cols;
            let col = idx % self.cols;
            let pos = Position::new(row, col);

            (pos, item)
        })
    }

    /// Creates an indexed iterator over a specified row of the grid
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_indexed_row(0);
    /// assert_eq!(iter.next(), Some((Position::new(0, 0), &'A')));
    /// assert_eq!(iter.next(), Some((Position::new(0, 1), &'B')));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_indexed_row(1);
    /// assert_eq!(iter.next(), Some((Position::new(1, 0), &'C')));
    /// assert_eq!(iter.next(), Some((Position::new(1, 1), &'D')));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_indexed_row(2);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_indexed_row(&self, row: usize) -> GridIndexedIter<'_, T> {
        GridIndexedIter::new_linear(self.iter_row(row))
    }

    /// Creates a mutable indexed iterator over a specified row of the grid
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let mut grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    ///
    /// {
    ///     let mut iter = grid.iter_indexed_row_mut(0);
    ///     assert_eq!(iter.next(), Some((Position::new(0, 0), &mut 'A')));
    ///     assert_eq!(iter.next(), Some((Position::new(0, 1), &mut 'B')));
    ///     assert_eq!(iter.next(), None);
    /// }
    ///
    /// {
    ///     let mut iter = grid.iter_indexed_row_mut(1);
    ///     assert_eq!(iter.next(), Some((Position::new(1, 0), &mut 'C')));
    ///     assert_eq!(iter.next(), Some((Position::new(1, 1), &mut 'D')));
    ///     assert_eq!(iter.next(), None);
    /// }
    ///
    /// {
    ///     let mut iter = grid.iter_indexed_row_mut(2);
    ///     assert_eq!(iter.next(), None);
    /// }
    /// ```
    pub fn iter_indexed_row_mut(&mut self, row: usize) -> GridIndexedIterMut<'_, T> {
        GridIndexedIterMut::new(self.iter_row_mut(row))
    }

    /// Creates an indexed iterator over the rows of the grid
    /// ```
    /// use puzzled_core::{grid, Order};
    ///
    /// let grid = grid![
    ///    [1, 2, 3],
    ///    [4, 5, 6],
    ///    [7, 8, 9]
    /// ];
    /// let mut row_sums = grid
    ///     .iter_indexed_lines(Order::Rows)
    ///     .map(|row| {
    ///         row.fold(0, |acc, (pos, val)| {
    ///             acc + 100 * val + 10 * pos.row + pos.col
    ///         })
    ///     });
    ///
    /// assert_eq!(row_sums.next(), Some(100 + 200 + 300 + 00 + 01 + 02));
    /// assert_eq!(row_sums.next(), Some(400 + 500 + 600 + 10 + 11 + 12));
    /// assert_eq!(row_sums.next(), Some(700 + 800 + 900 + 20 + 21 + 22));
    /// assert_eq!(row_sums.next(), None);
    /// ```
    pub fn iter_indexed_rows(&self) -> impl Iterator<Item = GridIndexedIter<'_, T>> {
        self.iter_rows().map(GridIndexedIter::new_linear)
    }

    /// Creates an indexed iterator over a specified column of the grid
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_indexed_col(0);
    /// assert_eq!(iter.next(), Some((Position::new(0, 0), &'A')));
    /// assert_eq!(iter.next(), Some((Position::new(1, 0), &'C')));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_indexed_col(1);
    /// assert_eq!(iter.next(), Some((Position::new(0, 1), &'B')));
    /// assert_eq!(iter.next(), Some((Position::new(1, 1), &'D')));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_indexed_col(2);
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_indexed_col(&self, col: usize) -> GridIndexedIter<'_, T> {
        GridIndexedIter::new_linear(self.iter_col(col))
    }

    /// Creates a mutable indexed iterator over a specified column of the grid
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let mut grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// {
    ///     let mut iter = grid.iter_indexed_col_mut(0);
    ///     assert_eq!(iter.next(), Some((Position::new(0, 0), &mut 'A')));
    ///     assert_eq!(iter.next(), Some((Position::new(1, 0), &mut 'C')));
    ///     assert_eq!(iter.next(), None);
    /// }
    /// {
    ///     let mut iter = grid.iter_indexed_col_mut(1);
    ///     assert_eq!(iter.next(), Some((Position::new(0, 1), &mut 'B')));
    ///     assert_eq!(iter.next(), Some((Position::new(1, 1), &mut 'D')));
    ///     assert_eq!(iter.next(), None);
    /// }
    /// {
    ///     let mut iter = grid.iter_indexed_col_mut(2);
    ///     assert_eq!(iter.next(), None);
    /// }
    /// ```
    pub fn iter_indexed_col_mut(&mut self, col: usize) -> GridIndexedIterMut<'_, T> {
        GridIndexedIterMut::new(self.iter_col_mut(col))
    }

    /// Creates an indexed iterator over the columns of the grid
    /// ```
    /// use puzzled_core::{grid, Order};
    ///
    /// let grid = grid![
    ///    [1, 2, 3],
    ///    [4, 5, 6],
    ///    [7, 8, 9]
    /// ];
    /// let mut col_sums = grid
    ///     .iter_indexed_lines(Order::Cols)
    ///     .map(|row| {
    ///         row.fold(0, |acc, (pos, val)| {
    ///             acc + 100 * val + 10 * pos.row + pos.col
    ///         })
    ///     });
    ///
    /// assert_eq!(col_sums.next(), Some(100 + 400 + 700 + 00 + 10 + 20));
    /// assert_eq!(col_sums.next(), Some(200 + 500 + 800 + 01 + 11 + 21));
    /// assert_eq!(col_sums.next(), Some(300 + 600 + 900 + 02 + 12 + 22));
    /// assert_eq!(col_sums.next(), None);
    /// ```
    pub fn iter_indexed_cols(&self) -> impl Iterator<Item = GridIndexedIter<'_, T>> {
        self.iter_cols().map(GridIndexedIter::new_linear)
    }

    /// Creates an indexed iterator over a specified [line](Line) of the grid
    /// ```
    /// use puzzled_core::{grid, Position, Line};
    ///
    /// let grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_indexed_line(Line::Row(0));
    /// assert_eq!(iter.next(), Some((Position::new(0, 0), &'A')));
    /// assert_eq!(iter.next(), Some((Position::new(0, 1), &'B')));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_indexed_line(Line::Col(0));
    /// assert_eq!(iter.next(), Some((Position::new(0, 0), &'A')));
    /// assert_eq!(iter.next(), Some((Position::new(1, 0), &'C')));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_indexed_line(&self, line: Line) -> GridIndexedIter<'_, T> {
        GridIndexedIter::new_linear(self.iter_line(line))
    }

    /// Creates a mutable indexed iterator over a specified [line](Line) of the grid
    /// ```
    /// use puzzled_core::{grid, Position, Line};
    ///
    /// let mut grid = grid![
    ///    ['A', 'B'],
    ///    ['C', 'D']
    /// ];
    /// let mut iter = grid.iter_indexed_line_mut(Line::Row(0));
    /// assert_eq!(iter.next(), Some((Position::new(0, 0), &mut 'A')));
    /// assert_eq!(iter.next(), Some((Position::new(0, 1), &mut 'B')));
    /// assert_eq!(iter.next(), None);
    ///
    /// let mut iter = grid.iter_indexed_line_mut(Line::Col(0));
    /// assert_eq!(iter.next(), Some((Position::new(0, 0), &mut 'A')));
    /// assert_eq!(iter.next(), Some((Position::new(1, 0), &mut 'C')));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_indexed_line_mut(&mut self, line: Line) -> GridIndexedIterMut<'_, T> {
        GridIndexedIterMut::new(self.iter_line_mut(line))
    }

    /// Creates an indexed iterator over the [lines](Line) of the grid in a specified [order](Order)
    /// ```
    /// use puzzled_core::{grid, Order};
    ///
    /// let grid = grid![
    ///    [1, 2, 3],
    ///    [4, 5, 6],
    ///    [7, 8, 9]
    /// ];
    /// let mut row_sums = grid
    ///     .iter_indexed_lines(Order::Rows)
    ///     .map(|row| {
    ///         row.fold(0, |acc, (pos, val)| {
    ///             acc + 100 * val + 10 * pos.row + pos.col
    ///         })
    ///     });
    ///
    /// assert_eq!(row_sums.next(), Some(100 + 200 + 300 + 00 + 01 + 02));
    /// assert_eq!(row_sums.next(), Some(400 + 500 + 600 + 10 + 11 + 12));
    /// assert_eq!(row_sums.next(), Some(700 + 800 + 900 + 20 + 21 + 22));
    /// assert_eq!(row_sums.next(), None);
    ///
    /// let mut col_sums = grid
    ///     .iter_indexed_lines(Order::Cols)
    ///     .map(|row| {
    ///         row.fold(0, |acc, (pos, val)| {
    ///             acc + 100 * val + 10 * pos.row + pos.col
    ///         })
    ///     });
    ///
    /// assert_eq!(col_sums.next(), Some(100 + 400 + 700 + 00 + 10 + 20));
    /// assert_eq!(col_sums.next(), Some(200 + 500 + 800 + 01 + 11 + 21));
    /// assert_eq!(col_sums.next(), Some(300 + 600 + 900 + 02 + 12 + 22));
    /// assert_eq!(col_sums.next(), None);
    /// ```
    pub fn iter_indexed_lines(&self, order: Order) -> impl Iterator<Item = GridIndexedIter<'_, T>> {
        self.iter_lines(order).map(GridIndexedIter::new_linear)
    }

    pub fn iter_indexed_segment(&self, pos: Position, dir: Direction) -> GridIndexedIter<'_, T> {
        GridIndexedIter::new_linear(self.iter_segment(pos, dir))
    }

    pub fn iter_indexed_segment_mut(
        &mut self,
        pos: Position,
        dir: Direction,
    ) -> GridIndexedIterMut<'_, T> {
        GridIndexedIterMut::new(self.iter_segment_mut(pos, dir))
    }
}
