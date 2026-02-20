use std::{
    iter::StepBy,
    marker::PhantomData,
    slice::{Iter, IterMut},
};

use crate::{Grid, Line, Order, Position};

type RowIter<'a, T> = Iter<'a, T>;
type RowIterMut<'a, T> = IterMut<'a, T>;

type ColIter<'a, T> = StepBy<Iter<'a, T>>;
type ColIterMut<'a, T> = StepBy<IterMut<'a, T>>;

impl<T> Grid<T> {
    /// Creates an iterator over the [positions](Position) of the grid
    ///
    /// The entries are traversed in [row-major](crate::Order::RowMajor) order.
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
    /// The entries are traversed in [row-major](crate::Order::RowMajor) order.
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
    /// The entries are traversed in [row-major](crate::Order::RowMajor) order.
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

struct PosIter<'a, T, I: 'a> {
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

struct PosIterMut<'a, T, I> {
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
    /// The entries are traversed in [row-major](crate::Order::RowMajor) order.
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
    /// The entries are traversed in [row-major](crate::Order::RowMajor) order.
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
    ///     .iter_indexed_lines(Order::RowMajor)
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
    ///     .iter_indexed_lines(Order::ColMajor)
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
    /// let row_sums: Vec<usize> = grid.iter_lines(Order::RowMajor).map(|row| row.sum()).collect();
    /// assert_eq!(row_sums, vec![6, 15, 24]);
    ///
    /// let col_sums: Vec<usize> = grid.iter_lines(Order::ColMajor).map(|col| col.sum()).collect();
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
    ///     .iter_indexed_lines(Order::RowMajor)
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
    ///     .iter_indexed_lines(Order::ColMajor)
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
