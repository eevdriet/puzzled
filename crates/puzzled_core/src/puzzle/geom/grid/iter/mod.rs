mod indexed;
mod line;

pub(crate) use indexed::*;

use std::{
    iter::StepBy,
    slice::{Iter, IterMut},
};

use crate::{Grid, Position};

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
