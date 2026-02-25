use crate::{Grid, PosIter, PosIterMut, Position, Square};

impl<T> Grid<Square<T>> {
    /// Get a reference to the [filled square](Square::Filled) at the given position
    ///
    /// Same as [`get`](Grid<Square>::get), but additionally checks whether the square is a cell
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let opt_grid = grid! (
    ///    [None, Some(1)],
    ///    [Some(2), None]
    /// );
    ///
    /// assert_eq!(opt_grid.get_fill(Position::new(0, 1)), Some(&1));
    /// assert_eq!(opt_grid.get_fill(Position::new(1, 1)), None);
    /// assert_eq!(opt_grid.get_fill(Position::new(2, 1)), None);
    /// ```
    pub fn get_fill(&self, pos: Position) -> Option<&T> {
        self.get(pos).and_then(|opt| opt.as_ref())
    }

    /// Get a mutable reference to the [filled square](Square::Filled) at the given position
    ///
    /// Same as [`get_mut`](Grid<Square>::get_mut), but additionally checks whether the square is a cell
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let mut opt_grid = grid! (
    ///    [None, Some(1)],
    ///    [Some(2), None]
    /// );
    ///
    /// assert_eq!(opt_grid.get_fill_mut(Position::new(0, 1)), Some(&mut 1));
    /// assert_eq!(opt_grid.get_fill_mut(Position::new(1, 1)), None);
    /// assert_eq!(opt_grid.get_fill_mut(Position::new(2, 1)), None);
    /// ```
    pub fn get_fill_mut(&mut self, pos: Position) -> Option<&mut T> {
        self.get_mut(pos).and_then(|opt| opt.as_mut())
    }

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
    /// let mut iter = opt_grid.iter_cells();
    ///
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_fills(&self) -> impl Iterator<Item = &T> {
        self.iter().filter_map(|a| a.as_ref())
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
    /// let mut iter = opt_grid.iter_cells_mut();
    ///
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_fills_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.iter_mut().filter_map(|square| square.as_mut())
    }

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
    pub fn iter_fills_indexed(&self) -> impl Iterator<Item = (Position, &T)> {
        let iter = self
            .data
            .iter()
            .enumerate()
            .filter_map(|(pos, square)| square.as_ref().map(|sq| (pos, sq)));

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
    pub fn iter_fills_indexed_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> {
        let iter = self
            .data
            .iter_mut()
            .enumerate()
            .filter_map(|(pos, square)| square.as_mut().map(|sq| (pos, sq)));

        PosIterMut::new(iter, self.cols, false)
    }
}
