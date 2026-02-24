mod indexed;
mod line;

use std::{
    iter::StepBy,
    slice::{Iter, IterMut},
};

use crate::{Grid, Line, Order, Position, puzzle::geom::grid::iter::indexed::PosIter};

pub(crate) type RowIter<'a, T> = Iter<'a, T>;
pub(crate) type RowIterMut<'a, T> = IterMut<'a, T>;

pub(crate) type ColIter<'a, T> = StepBy<Iter<'a, T>>;
pub(crate) type ColIterMut<'a, T> = StepBy<IterMut<'a, T>>;

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
    pub fn iter_row(&self, row: usize) -> RowIter<'_, T> {
        if row >= self.rows {
            return [].iter();
        }

        let start = row * self.cols;
        let end = start + self.cols;

        self.data[start..end].iter()
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
    pub fn iter_row_mut(&mut self, row: usize) -> RowIterMut<'_, T> {
        if row >= self.rows {
            return [].iter_mut();
        }

        let start = row * self.cols;
        let end = start + self.cols;

        self.data[start..end].iter_mut()
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
    pub fn iter_rows(&self) -> impl Iterator<Item = RowIter<'_, T>> {
        (0..self.rows).map(move |row| self.iter_row(row))
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
    pub fn iter_col(&self, col: usize) -> ColIter<'_, T> {
        let iter = if col >= self.cols {
            [].iter()
        } else {
            let start = col;
            self.data[start..].iter()
        };

        iter.step_by(self.cols)
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
    pub fn iter_col_mut(&mut self, col: usize) -> ColIterMut<'_, T> {
        let iter = if col >= self.cols {
            [].iter_mut()
        } else {
            let start = col;
            self.data[start..].iter_mut()
        };

        iter.step_by(self.cols)
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
    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.cols).map(move |col| self.iter_col(col))
    }
}

impl<T> Grid<Option<T>> {
    /// Returns an iterator over the filled squares of the puzzle.
    ///
    /// The filled squares are traversed in row-major order.
    /// ```
    /// use puzzled_core::grid;
    ///
    /// let opt_grid = grid! (
    ///    [None, Some(1)],
    ///    [Some(2), None]
    /// );
    /// let mut iter = opt_grid.iter_fills();
    ///
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_fills(&self) -> impl Iterator<Item = &T> {
        self.iter().filter_map(|opt| opt.as_ref())
    }

    /// Returns a mutable iterator over the filled squares of the puzzle.
    ///
    /// The filled squares are traversed in row-major order.
    /// ```
    /// use puzzled_core::grid;
    ///
    /// let mut opt_grid = grid! (
    ///    [None, Some(1)],
    ///    [Some(2), None]
    /// );
    /// let mut iter = opt_grid.iter_fills_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_fills_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.iter_mut().filter_map(|opt| opt.as_mut())
    }
}

enum LinePosIter<'a, T, I, J> {
    Row(PosIter<'a, T, I>),
    Col(PosIter<'a, T, J>),
    Empty,
}

impl<'a, T, I, J> Iterator for LinePosIter<'a, T, I, J>
where
    I: Iterator<Item = (usize, &'a T)>,
    J: Iterator<Item = (usize, &'a T)>,
    T: 'a,
{
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Row(iter) => iter.next(),
            Self::Col(iter) => iter.next(),
            _ => None,
        }
    }
}

impl<T> Grid<T> {
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
    pub fn iter_indexed_line(&self, line: Line) -> impl Iterator<Item = (Position, &T)> {
        match line {
            Line::Row(row) if row < self.rows => {
                let iter = self
                    .data
                    .iter()
                    .enumerate()
                    .skip(row * self.cols)
                    .take(self.cols);

                LinePosIter::Row(PosIter::new(iter, self.cols, false))
            }
            Line::Col(col) if col < self.cols => {
                let iter = self.data.iter().enumerate().skip(col).step_by(self.cols);

                LinePosIter::Col(PosIter::new(iter, self.cols, false))
            }
            _ => LinePosIter::Empty,
        }
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
    pub fn iter_indexed_lines(
        &self,
        order: Order,
    ) -> impl Iterator<Item = impl Iterator<Item = (Position, &T)>> {
        let range = match order {
            Order::Rows => 0..self.rows,
            Order::Cols => 0..self.cols,
        };

        let lines = range.map(move |line| match order {
            Order::Rows => Line::Row(line),
            Order::Cols => Line::Col(line),
        });

        lines.map(|line| self.iter_indexed_line(line))
    }
}
