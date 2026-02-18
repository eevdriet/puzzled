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
    pub fn positions(&self) -> impl Iterator<Item = Position> {
        (0..self.data.len()).map(move |idx| self.position(idx).expect("Position should be valid"))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn iter_row(&self, row: usize) -> RowIter<'_, T> {
        let start = row * self.cols;
        let end = start + self.cols;

        self.data[start..end].iter()
    }

    pub fn iter_row_mut(&mut self, row: usize) -> RowIterMut<'_, T> {
        let start = row * self.cols;
        let end = start + self.cols;

        self.data[start..end].iter_mut()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = RowIter<'_, T>> {
        (0..self.rows).map(move |row| self.iter_row(row))
    }

    pub fn iter_col(&self, col: usize) -> ColIter<'_, T> {
        let start = col;
        self.data[start..].iter().step_by(self.cols)
    }

    pub fn iter_col_mut(&mut self, col: usize) -> ColIterMut<'_, T> {
        let start = col;
        self.data[start..].iter_mut().step_by(self.cols)
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = ColIter<'_, T>> {
        (0..self.cols).map(move |col| self.iter_col(col))
    }
}

pub struct PosIter<'a, T, I: 'a> {
    iter: I,
    cols: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, I> PosIter<'a, T, I> {
    pub fn new(iter: I, cols: usize) -> Self {
        Self {
            iter,
            cols,
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
        let (idx, val) = self.iter.next()?;

        let pos = Position {
            row: idx / self.cols,
            col: idx % self.cols,
        };

        Some((pos, val))
    }
}

pub struct PosIterMut<'a, T, I> {
    iter: I,
    cols: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, I> PosIterMut<'a, T, I> {
    pub fn new(iter: I, cols: usize) -> Self {
        Self {
            iter,
            cols,
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
        let (idx, val) = self.iter.next()?;

        let pos = Position {
            row: idx / self.cols,
            col: idx % self.cols,
        };

        Some((pos, val))
    }
}

impl<T> Grid<T> {
    pub fn indexed_iter(&self) -> impl Iterator<Item = (Position, &T)> {
        let iter = self.data.iter().enumerate();

        PosIter::new(iter, self.cols)
    }

    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> {
        let iter = self.data.iter_mut().enumerate();

        PosIterMut::new(iter, self.cols)
    }

    pub fn indexed_iter_row(&self, row: usize) -> impl Iterator<Item = (Position, &T)> {
        let iter = self
            .data
            .iter()
            .enumerate()
            .skip(row * self.cols)
            .take(self.cols);

        PosIter::new(iter, self.cols)
    }

    pub fn indexed_iter_row_mut(&mut self, row: usize) -> impl Iterator<Item = (Position, &mut T)> {
        let iter = self
            .data
            .iter_mut()
            .enumerate()
            .skip(row * self.cols)
            .take(self.cols);

        PosIterMut::new(iter, self.cols)
    }

    pub fn indexed_iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = (Position, &T)>> {
        (0..self.rows).map(move |row| self.indexed_iter_row(row))
    }

    pub fn indexed_iter_col(&self, col: usize) -> impl Iterator<Item = (Position, &T)> {
        let iter = self.data.iter().enumerate().skip(col).step_by(self.cols);

        PosIter::new(iter, self.cols)
    }

    pub fn indexed_iter_col_mut(&mut self, col: usize) -> impl Iterator<Item = (Position, &mut T)> {
        let iter = self
            .data
            .iter_mut()
            .enumerate()
            .skip(col)
            .step_by(self.cols);

        PosIterMut::new(iter, self.cols)
    }

    pub fn indexed_iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = (Position, &T)>> {
        (0..self.cols).map(move |col| self.indexed_iter_row(col))
    }
}

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

impl<T> Grid<T> {
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

    pub fn iter_lines<'a>(&'a self, order: Order) -> impl Iterator<Item = LineIter<'a, T>> {
        let lines = match order {
            Order::RowMajor => 0..self.rows,
            Order::ColMajor => 0..self.cols,
        };

        lines.map(move |line| match order {
            Order::RowMajor => {
                let iter = self.iter_row(line);
                LineIter::Row(iter)
            }
            Order::ColMajor => {
                let iter = self.iter_col(line);
                LineIter::Col(iter)
            }
        })
    }
}

pub enum LinePosIter<'a, T, I, J> {
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
    pub fn indexed_iter_line(&self, line: Line) -> impl Iterator<Item = (Position, &T)> {
        match line {
            Line::Row(row) if row < self.rows => {
                let iter = self
                    .data
                    .iter()
                    .enumerate()
                    .skip(row * self.cols)
                    .take(self.cols);

                LinePosIter::Row(PosIter::new(iter, self.cols))
            }
            Line::Col(col) if col < self.cols => {
                let iter = self.data.iter().enumerate().skip(col).step_by(self.cols);

                LinePosIter::Col(PosIter::new(iter, self.cols))
            }
            _ => LinePosIter::Empty,
        }
    }

    pub fn indexed_iter_lines(
        &self,
        order: Order,
    ) -> impl Iterator<Item = impl Iterator<Item = (Position, &T)>> {
        let range = match order {
            Order::RowMajor => 0..self.rows,
            Order::ColMajor => 0..self.cols,
        };

        let lines = range.map(move |line| match order {
            Order::RowMajor => Line::Row(line),
            Order::ColMajor => Line::Col(line),
        });

        lines.map(|line| self.indexed_iter_line(line))
    }
}
