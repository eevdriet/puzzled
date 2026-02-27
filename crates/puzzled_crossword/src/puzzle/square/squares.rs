use std::ops;

use puzzled_core::{Grid, Offset, Position, Square};

use crate::{ClueDirection, Crossword, CrosswordSquare};

pub type Squares = Grid<CrosswordSquare>;

pub trait CrosswordSquares {
    fn can_clue_start_in_dir(&self, pos: Position, dir: ClueDirection) -> bool;
    fn find_clue_len(&self, pos: Position, dir: ClueDirection) -> u8;
}

impl<T> CrosswordSquares for Grid<Square<T>> {
    fn can_clue_start_in_dir(&self, pos: Position, dir: ClueDirection) -> bool {
        let is_blank = |pos: Position| self[pos].as_ref().is_none();

        if is_blank(pos) {
            return false;
        }

        match dir {
            ClueDirection::Across => pos.col == 0 || is_blank(pos + Offset::LEFT),
            ClueDirection::Down => pos.row == 0 || is_blank(pos + Offset::UP),
        }
    }

    fn find_clue_len(&self, pos: Position, dir: ClueDirection) -> u8 {
        let offset = match dir {
            ClueDirection::Across => Offset::RIGHT,
            ClueDirection::Down => Offset::DOWN,
        };

        (0..)
            .scan(pos, |acc, _| {
                let square = self.get(*acc)?;
                *acc += offset;

                Some(square)
            })
            .count() as u8
    }
}

impl ops::Index<Position> for Crossword {
    type Output = CrosswordSquare;

    /// Index the puzzle to retrieve a reference to the [square](Square) at the given [position](Position).
    /// ```
    /// use puzzled::crossword::{crossword, Position, CrosswordCell};
    ///
    /// let mut puzzle = crossword! (
    ///    [A .]
    ///    [C D]
    /// );
    /// let mut puzzle2 = crossword! (
    ///    [A B]
    ///    [C D]
    /// );
    ///
    /// let pos = Position::new(0, 1);
    /// puzzle[pos] = Some(CrosswordCell::letter('B'));
    /// assert_eq!(puzzle, puzzle2);
    /// ```
    /// # Panics
    /// Panics if the given `pos` is out of bounds, i.e. `pos.row >= puzzle.rows() || pos.col >= puzzle.cols()`.
    /// ```should_panic
    /// use puzzled::crossword::{crossword, Position, CrosswordCell};
    ///
    /// let mut puzzle = crossword! (
    ///    [A .]
    ///    [C D]
    /// );
    ///
    /// let pos = Position::new(2, 1);
    /// puzzle[pos] = Some(CrosswordCell::letter('E'));
    /// ```
    fn index(&self, pos: Position) -> &Self::Output {
        &self.squares[pos]
    }
}

impl ops::IndexMut<Position> for Crossword {
    /// Index the puzzle to retrieve a mutable reference to the [square](Square) at the given [position](Position).
    /// ```
    /// use puzzled::crossword::{crossword, Position, CrosswordCell};
    ///
    /// let mut puzzle = crossword! (
    ///    [A .]
    ///    [C D]
    /// );
    /// let mut puzzle2 = crossword! (
    ///    [A B]
    ///    [C D]
    /// );
    ///
    /// let pos = Position::new(0, 1);
    /// puzzle[pos] = Some(CrosswordCell::letter('B'));
    /// assert_eq!(puzzle, puzzle2);
    /// ```
    /// # Panics
    /// Panics if the given `pos` is out of bounds, i.e. `pos.row >= puzzle.rows() || pos.col >= puzzle.cols()`.
    /// ```should_panic
    /// use puzzled::crossword::{crossword, Position, CrosswordCell};
    ///
    /// let mut puzzle = crossword! (
    ///    [A .]
    ///    [C D]
    /// );
    ///
    /// let pos = Position::new(2, 1);
    /// puzzle[pos] = Some(CrosswordCell::letter('E'));
    /// ```
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.squares[pos]
    }
}
