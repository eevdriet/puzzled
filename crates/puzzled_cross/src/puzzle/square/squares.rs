use std::ops;

use puzzled_core::{Grid, Offset, Position};

use crate::{Cell, Direction, Puzzle, Square};

/// Collection type of all [squares](Square) in a [puzzle](crate::Puzzle)
///
/// A [grid](Grid) is used to represent the squares.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Squares(Grid<Square>);

impl Squares {
    pub fn new(grid: Grid<Square>) -> Self {
        Self(grid)
    }

    pub(crate) fn starts_in_dir(&self, pos: Position, dir: Direction) -> bool {
        let is_blank = |pos: Position| {
            self.get(pos)
                .is_none_or(|square| !matches!(square, Square::White(_)))
        };

        if is_blank(pos) {
            return false;
        }

        match dir {
            Direction::Across => pos.col == 0 || is_blank(pos + Offset::LEFT),
            Direction::Down => pos.row == 0 || is_blank(pos + Offset::UP),
        }
    }

    pub(crate) fn find_playable_len(&self, pos: Position, dir: Direction) -> u8 {
        let offset = match dir {
            Direction::Across => Offset::RIGHT,
            Direction::Down => Offset::DOWN,
        };

        (0..)
            .scan(pos, |acc, _| {
                let square = self.get(*acc)?;

                if matches!(square, Square::White(_)) {
                    let current = *acc;
                    *acc += offset;

                    Some(current)
                } else {
                    None
                }
            })
            .count() as u8
    }

    /// Get a reference to the [cell](Cell) at the given position
    ///
    /// Same as [`get`](Self::get), but additionally checks whether the square is a cell
    /// ```
    /// use puzzled_crossword::{cell, Position, puzzle, Solution::*};
    ///
    /// let puzzle = puzzle! (
    ///    [. B]
    ///    [D .]
    /// );
    /// assert_eq!(puzzle.squares().get_cell(Position::new(0, 1)), Some(&cell!('B')));
    /// assert_eq!(puzzle.squares().get_cell(Position::new(1, 1)), None);
    /// assert_eq!(puzzle.squares().get_cell(Position::new(2, 1)), None);
    /// ```
    pub fn get_cell(&self, pos: Position) -> Option<&Cell> {
        match self.get(pos) {
            Some(Square::White(cell)) => Some(cell),
            _ => None,
        }
    }

    /// Get a mutable reference to the [square](Cell) at the given position
    ///
    /// Same as [`get_mut`](Self::get_mut), but additionally checks whether the square is a cell
    /// ```
    /// use puzzled_crossword::{cell, Position, puzzle, Solution::*};
    ///
    /// let mut puzzle = puzzle! (
    ///    [. B]
    ///    [D .]
    /// );
    /// assert_eq!(puzzle.squares_mut().get_cell_mut(Position::new(0, 1)), Some(&mut cell!('B')));
    /// assert_eq!(puzzle.squares_mut().get_cell_mut(Position::new(1, 1)), None);
    /// assert_eq!(puzzle.squares_mut().get_cell_mut(Position::new(2, 1)), None);
    /// ```
    pub fn get_cell_mut(&mut self, pos: Position) -> Option<&mut Cell> {
        match self.get_mut(pos) {
            Some(Square::White(cell)) => Some(cell),
            _ => None,
        }
    }

    /// Returns an iterator over the cells of the puzzle.
    ///
    /// The cells are traversed in row-major order.
    /// ```
    /// use puzzled_crossword::{Cell, puzzle, Square, Solution::*};
    ///
    /// let puzzle = puzzle! (
    ///    [A .]
    ///    [. D]
    /// );
    /// let mut iter = puzzle.squares().iter_cells();
    /// assert_eq!(iter.next(), Some(&Cell::new(Letter('A'))));
    /// assert_eq!(iter.next(), Some(&Cell::new(Letter('D'))));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_cells(&self) -> impl Iterator<Item = &Cell> {
        self.iter().filter_map(|square| match square {
            Square::Black => None,
            Square::White(cell) => Some(cell),
        })
    }

    /// Returns a mutable iterator over the cells of the puzzle.
    ///
    /// The cells are traversed in row-major order.
    /// ```
    /// use puzzled_crossword::{Cell, puzzle, Square, Solution::*};
    ///
    /// let mut puzzle = puzzle! (
    ///    [A .]
    ///    [. D]
    /// );
    /// let mut iter = puzzle.squares_mut().iter_cells_mut();
    /// assert_eq!(iter.next(), Some(&mut Cell::new(Letter('A'))));
    /// assert_eq!(iter.next(), Some(&mut Cell::new(Letter('D'))));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.iter_mut().filter_map(|square| match square {
            Square::Black => None,
            Square::White(cell) => Some(cell),
        })
    }
}

impl ops::Deref for Squares {
    type Target = Grid<Square>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Squares {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ops::Index<Position> for Puzzle {
    type Output = Square;

    /// Index the puzzle to retrieve a reference to the [square](Square) at the given [position](Position).
    /// ```
    /// use puzzled_crossword::{Position, puzzle, Square};
    ///
    /// let mut puzzle = puzzle! (
    ///    [A .]
    ///    [C D]
    /// );
    /// let mut puzzle2 = puzzle! (
    ///    [A B]
    ///    [C D]
    /// );
    ///
    /// let pos = Position::new(0, 1);
    /// puzzle[pos] = Square::letter('B');
    /// assert_eq!(puzzle, puzzle2);
    /// ```
    /// # Panics
    /// Panics if the given `pos` is out of bounds, i.e. `pos.row >= puzzle.rows() || pos.col >= puzzle.cols()`.
    /// ```should_panic
    /// use puzzled_crossword::{Position, puzzle, Square};
    ///
    /// let mut puzzle = puzzle! (
    ///    [A .]
    ///    [C D]
    /// );
    ///
    /// let pos = Position::new(2, 1);
    /// puzzle[pos] = Square::letter('E');
    /// ```
    fn index(&self, pos: Position) -> &Self::Output {
        &self.squares[pos]
    }
}

impl ops::IndexMut<Position> for Puzzle {
    /// Index the puzzle to retrieve a mutable reference to the [square](Square) at the given [position](Position).
    /// ```
    /// use puzzled_crossword::{Position, puzzle, Square};
    ///
    /// let mut puzzle = puzzle! (
    ///    [A .]
    ///    [C D]
    /// );
    /// let mut puzzle2 = puzzle! (
    ///    [A B]
    ///    [C D]
    /// );
    ///
    /// let pos = Position::new(0, 1);
    /// puzzle[pos] = Square::letter('B');
    /// assert_eq!(puzzle, puzzle2);
    /// ```
    /// # Panics
    /// Panics if the given `pos` is out of bounds, i.e. `pos.row >= puzzle.rows() || pos.col >= puzzle.cols()`.
    /// ```should_panic
    /// use puzzled_crossword::{Position, puzzle, Square};
    ///
    /// let mut puzzle = puzzle! (
    ///    [A .]
    ///    [C D]
    /// );
    ///
    /// let pos = Position::new(2, 1);
    /// puzzle[pos] = Square::letter('E');
    /// ```
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.squares[pos]
    }
}
