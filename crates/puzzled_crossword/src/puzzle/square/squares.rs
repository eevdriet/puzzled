use std::ops;

use derive_more::{Deref, DerefMut};
use puzzled_core::{Grid, Offset, Position};

use crate::{ClueDirection, Crossword, CrosswordSquare};

#[derive(Debug, PartialEq, Eq, Deref, DerefMut, Clone)]
pub struct Squares(pub(crate) Grid<CrosswordSquare>);

impl Squares {
    pub fn new(squares: Grid<CrosswordSquare>) -> Self {
        Self(squares)
    }

    pub fn can_clue_start_in_dir(&self, pos: Position, dir: ClueDirection) -> bool {
        let is_blank = |pos: Position| self[pos].is_none();

        if is_blank(pos) {
            return false;
        }

        match dir {
            ClueDirection::Across => pos.col == 0 || is_blank(pos + Offset::LEFT),
            ClueDirection::Down => pos.row == 0 || is_blank(pos + Offset::UP),
        }
    }

    pub fn find_clue_len(&self, pos: Position, dir: ClueDirection) -> u8 {
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

#[cfg(feature = "serde")]
mod serde_impl {
    use puzzled_core::Grid;
    use serde::{Deserialize, Serialize};

    use crate::{CrosswordSquare, Squares};

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Squares {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Squares {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let squares = Grid::<CrosswordSquare>::deserialize(deserializer)?;
            let squares = Squares::new(squares);

            Ok(squares)
        }
    }
}
