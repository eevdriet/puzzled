mod indexed;
mod linear;
mod positions;

pub use indexed::*;
pub use linear::*;
pub use positions::*;

use crate::{Grid, Position};

#[derive(Debug, Clone)]
pub enum GridIter<'a, T> {
    Linear(GridLinearIter<'a, T>),
    Positions(GridPositionsIter<'a, T>),
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
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
