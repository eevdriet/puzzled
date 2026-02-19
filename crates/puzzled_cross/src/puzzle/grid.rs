use std::ops;

use crate::{Cell, Direction, Grid, Offset, Position, Crossword, Square};

/// Collection type of all [squares](Square) in a [puzzle](crate::Crossword)
///
/// A [grid](Grid) is used to represent the squares.
pub type Squares = Grid<Square>;

pub(crate) trait GridExtension {
    fn starts_in_dir(&self, pos: Position, dir: Direction) -> bool;

    fn find_playable_len(&self, pos: Position, dir: Direction) -> u8;

    fn get_cell(&self, pos: Position) -> Option<&Cell>;

    fn get_cell_mut(&mut self, pos: Position) -> Option<&mut Cell>;

    fn iter_cells(&self) -> impl Iterator<Item = &Cell>;

    fn iter_cells_mut(&mut self) -> impl Iterator<Item = &mut Cell>;
}

/// # Crossword squares
impl Crossword {
    /// Number of rows (height) in the puzzle.
    ///
    /// Note that this includes blank squares
    /// ```
    /// use puzzled_crossword::crossword;
    ///
    /// let puzzle = crossword! (
    ///    [A B C]
    ///    [D E F]
    /// );
    /// assert_eq!(puzzle.rows(), 2);
    /// assert_eq!(puzzle.cols(), 3);
    /// ```
    pub fn rows(&self) -> usize {
        self.squares.rows()
    }

    /// Number of columns (width) in the puzzle.
    ///
    /// Note that this includes blank squares
    /// ```
    /// use puzzled_crossword::crossword;
    ///
    /// let puzzle = crossword! (
    ///    [A B C]
    ///    [D E F]
    /// );
    /// assert_eq!(puzzle.rows(), 2);
    /// assert_eq!(puzzle.cols(), 3);
    /// ```
    pub fn cols(&self) -> usize {
        self.squares.cols()
    }
}
