pub trait Reveal {
    fn reveal(&mut self);
}

use crate::{Grid, Position};

impl<R> Grid<R>
where
    R: Reveal,
{
    /// Try to reveal a [cell](crate::Cell) at a given [position](Position).
    /// Returns whether successful, i.e. the [square](Square) at the position is in-bounds and a cell
    /// ```
    /// use puzzled::crossword::{crossword, Position, Reveal};
    ///
    /// let mut puzzle = crossword! (
    ///    [A B C]
    ///    [D E F]
    ///    [G H I]
    /// );
    ///
    /// let pos = Position::new(1, 1);
    /// assert!(puzzle.squares_mut().reveal(pos));
    /// assert!(
    ///     puzzle
    ///         .squares()
    ///         .get_fill(pos)
    ///         .is_some_and(|cell| cell.is_revealed())
    /// );
    ///
    /// let pos2 = Position::new(10, 10);
    /// assert!(!puzzle.squares_mut().reveal(pos2));
    /// ```
    pub fn reveal(&mut self, pos: Position) -> bool {
        // Try to get the square to reveal
        let Some(cell) = self.get_mut(pos) else {
            return false;
        };

        cell.reveal();
        true
    }

    pub fn reveal_all(&mut self) {
        for cell in self.iter_mut() {
            cell.reveal();
        }
    }
}

impl<R> Grid<Option<R>>
where
    R: Reveal,
{
    /// Try to reveal a [cell](crate::Cell) at a given [position](Position).
    /// Returns whether successful, i.e. the [square](Square) at the position is in-bounds and a cell
    /// ```
    /// use puzzled::crossword::{crossword, Position, Reveal};
    ///
    /// let mut puzzle = crossword! (
    ///    [A B C]
    ///    [D E F]
    ///    [G H I]
    /// );
    ///
    /// let pos = Position::new(1, 1);
    /// assert!(puzzle.squares_mut().reveal(pos));
    /// assert!(
    ///     puzzle
    ///         .squares()
    ///         .get_fill(pos)
    ///         .is_some_and(|cell| cell.is_revealed())
    /// );
    ///
    /// let pos2 = Position::new(10, 10);
    /// assert!(!puzzle.squares_mut().reveal(pos2));
    /// ```
    pub fn reveal(&mut self, pos: Position) -> bool {
        // Try to get the square to reveal
        let Some(cell) = self.get_fill_mut(pos) else {
            return false;
        };

        cell.reveal();
        true
    }

    pub fn reveal_all(&mut self) {
        for cell in self.iter_fills_mut() {
            cell.reveal();
        }
    }
}
