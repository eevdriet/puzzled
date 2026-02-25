use std::marker::PhantomData;

use crate::{Grid, Position};

pub(crate) struct PosIter<'a, T, I: 'a> {
    iter: I,
    cols: usize,
    is_empty: bool,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, I> PosIter<'a, T, I> {
    pub fn new(iter: I, cols: usize, is_empty: bool) -> Self {
        Self {
            iter,
            cols,
            is_empty,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, I> Iterator for PosIter<'a, T, I>
where
    I: Iterator<Item = (usize, &'a T)>,
{
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty {
            return None;
        }

        let (idx, val) = self.iter.next()?;
        let pos = Position::from_row_order(idx, self.cols);

        Some((pos, val))
    }
}

pub(crate) struct PosIterMut<'a, T, I> {
    iter: I,
    cols: usize,
    is_empty: bool,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, I> PosIterMut<'a, T, I> {
    pub fn new(iter: I, cols: usize, is_empty: bool) -> Self {
        Self {
            iter,
            cols,
            is_empty,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, I> Iterator for PosIterMut<'a, T, I>
where
    I: Iterator<Item = (usize, &'a mut T)>,
{
    type Item = (Position, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty {
            return None;
        }

        let (idx, val) = self.iter.next()?;
        let pos = Position::from_row_order(idx, self.cols);

        Some((pos, val))
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
        let iter = self.data.iter().enumerate();

        PosIter::new(iter, self.cols, false)
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
        let iter = self.data.iter_mut().enumerate();

        PosIterMut::new(iter, self.cols, false)
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
    pub fn iter_indexed_row(&self, row: usize) -> impl Iterator<Item = (Position, &T)> {
        let is_empty = row >= self.rows;
        let iter = self
            .data
            .iter()
            .enumerate()
            .skip(row * self.cols)
            .take(self.cols);

        PosIter::new(iter, self.cols, is_empty)
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
    pub fn iter_indexed_row_mut(&mut self, row: usize) -> impl Iterator<Item = (Position, &mut T)> {
        let is_empty = row >= self.rows;
        let iter = self
            .data
            .iter_mut()
            .enumerate()
            .skip(row * self.cols)
            .take(self.cols);

        PosIterMut::new(iter, self.cols, is_empty)
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
    pub fn iter_indexed_rows(&self) -> impl Iterator<Item = impl Iterator<Item = (Position, &T)>> {
        (0..self.rows).map(move |row| self.iter_indexed_row(row))
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
    pub fn iter_indexed_col(&self, col: usize) -> impl Iterator<Item = (Position, &T)> {
        let is_empty = col >= self.cols;
        let iter = self
            .data
            .iter()
            .enumerate()
            .skip(col)
            .step_by(self.cols)
            .take(self.rows);

        PosIter::new(iter, self.cols, is_empty)
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
    pub fn iter_indexed_col_mut(&mut self, col: usize) -> impl Iterator<Item = (Position, &mut T)> {
        let is_empty = col >= self.cols;

        let iter = self
            .data
            .iter_mut()
            .enumerate()
            .skip(col)
            .step_by(self.cols);

        PosIterMut::new(iter, self.cols, is_empty)
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
    pub fn iter_indexed_cols(&self) -> impl Iterator<Item = impl Iterator<Item = (Position, &T)>> {
        (0..self.cols).map(move |col| self.iter_indexed_row(col))
    }
}
