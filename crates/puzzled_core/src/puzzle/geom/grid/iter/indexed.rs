use crate::{Grid, Iter, IterMut, Line, LineSegment, Order, Position};

pub struct IndexedIter<'a, T> {
    inner: Iter<'a, T>,
}

impl<'a, T> IndexedIter<'a, T> {
    pub fn new(iter: Iter<'a, T>) -> Self {
        Self { inner: iter }
    }
}

impl<'a, T> Iterator for IndexedIter<'a, T> {
    type Item = (Position, &'a T);

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

impl<'a, T> DoubleEndedIterator for IndexedIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.inner.remaining == 0 {
            return None;
        }

        let pos = self.inner.front;
        let item = self.inner.next_back()?;

        Some((pos, item))
    }
}

impl<'a, T> ExactSizeIterator for IndexedIter<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct IndexedIterMut<'a, T> {
    inner: IterMut<'a, T>,
}

impl<'a, T> IndexedIterMut<'a, T> {
    pub fn new(iter: IterMut<'a, T>) -> Self {
        Self { inner: iter }
    }
}

impl<'a, T> Iterator for IndexedIterMut<'a, T> {
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

impl<'a, T> DoubleEndedIterator for IndexedIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.inner.remaining == 0 {
            return None;
        }

        let pos = self.inner.front;
        let item = self.inner.next_back()?;

        Some((pos, item))
    }
}

impl<'a, T> ExactSizeIterator for IndexedIterMut<'a, T> {
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
    pub fn iter_indexed_row(&self, row: usize) -> IndexedIter<'_, T> {
        IndexedIter::new(self.iter_row(row))
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
    pub fn iter_indexed_row_mut(&mut self, row: usize) -> IndexedIterMut<'_, T> {
        IndexedIterMut::new(self.iter_row_mut(row))
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
    pub fn iter_indexed_rows(&self) -> impl Iterator<Item = IndexedIter<'_, T>> {
        self.iter_rows().map(IndexedIter::new)
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
    pub fn iter_indexed_col(&self, col: usize) -> IndexedIter<'_, T> {
        IndexedIter::new(self.iter_col(col))
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
    pub fn iter_indexed_col_mut(&mut self, col: usize) -> IndexedIterMut<'_, T> {
        IndexedIterMut::new(self.iter_col_mut(col))
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
    pub fn iter_indexed_cols(&self) -> impl Iterator<Item = IndexedIter<'_, T>> {
        self.iter_cols().map(IndexedIter::new)
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
    pub fn iter_indexed_line(&self, line: Line) -> IndexedIter<'_, T> {
        IndexedIter::new(self.iter_line(line))
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
    pub fn iter_indexed_line_mut(&mut self, line: Line) -> IndexedIterMut<'_, T> {
        IndexedIterMut::new(self.iter_line_mut(line))
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
    pub fn iter_indexed_lines(&self, order: Order) -> impl Iterator<Item = IndexedIter<'_, T>> {
        self.iter_lines(order).map(IndexedIter::new)
    }

    pub fn iter_indexed_segment(&self, segment: &LineSegment) -> IndexedIter<'_, T> {
        IndexedIter::new(self.iter_segment(segment))
    }

    pub fn iter_indexed_segment_mut(&mut self, segment: &LineSegment) -> IndexedIterMut<'_, T> {
        IndexedIterMut::new(self.iter_segment_mut(segment))
    }
}
