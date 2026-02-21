use std::ops;

use puzzled_core::{Grid, Offset, Position};

use crate::{Cell, Crossword, Direction, Square};

pub type Squares = Grid<Square>;

pub(crate) trait SquaresExtension {
    fn starts_in_dir(&self, pos: Position, dir: Direction) -> bool;
    fn find_playable_len(&self, pos: Position, dir: Direction) -> u8;
}

#[cfg(feature = "serde")]
pub(crate) trait SquaresSerdeExtension
where
    Self: Sized,
{
    fn from_data(data: SquaresData, cols: usize) -> Result<Self, String>;
    fn to_data(&self) -> SquaresData;
}

impl SquaresExtension for Squares {
    fn starts_in_dir(&self, pos: Position, dir: Direction) -> bool {
        let is_blank = |pos: Position| self.get_fill(pos).is_none();

        if is_blank(pos) {
            return false;
        }

        match dir {
            Direction::Across => pos.col == 0 || is_blank(pos + Offset::LEFT),
            Direction::Down => pos.row == 0 || is_blank(pos + Offset::UP),
        }
    }

    fn find_playable_len(&self, pos: Position, dir: Direction) -> u8 {
        let offset = match dir {
            Direction::Across => Offset::RIGHT,
            Direction::Down => Offset::DOWN,
        };

        (0..)
            .scan(pos, |acc, _| {
                let square = self.get_fill(*acc)?;
                *acc += offset;

                Some(square)
            })
            .count() as u8
    }
}

#[cfg(feature = "serde")]
impl SquaresSerdeExtension for Squares {
    fn from_data(data: SquaresData, cols: usize) -> Result<Self, String> {
        use crate::{CellStyle, EMPTY_SQUARE};

        let SquaresData {
            solution,
            state,
            styles,
            ..
        } = data;

        // Default construct missing state/styles and verify their length
        let len = solution.len();
        let state = state.unwrap_or(vec![String::new(); len]);
        let styles = styles.unwrap_or(vec![CellStyle::EMPTY; len]);

        if state.len() != len {
            return Err(format!(
                "The state grid has a different length ({}) than the solution grid ({len})",
                state.len()
            ));
        }
        if styles.len() != len {
            return Err(format!(
                "The styles grid has a different length ({}) than the solution grid ({len})",
                styles.len()
            ));
        }

        let squares = solution
            .into_iter()
            .zip(state.into_iter().zip(styles))
            .map(|(solution, (entry, style))| {
                if solution == EMPTY_SQUARE.to_string() {
                    return None;
                }

                let mut cell = Cell::new_styled(solution.into(), style);
                if !entry.is_empty() {
                    cell.enter(entry);
                }

                Some(cell)
            })
            .collect();

        let squares: Squares = Squares::from_vec(squares, cols).ok_or(format!(
            "Grid length {len} does not divide the number of columns {cols}"
        ))?;

        Ok(squares)
    }

    fn to_data(&self) -> SquaresData {
        use crate::{CellStyle, EMPTY_SQUARE};

        let solution: Vec<_> = self
            .iter()
            .map(|square| match square {
                Some(cell) => cell.solution().to_string(),
                _ => EMPTY_SQUARE.to_string(),
            })
            .collect();

        // State
        let has_state = self.iter_fills().any(|cell| cell.entry().is_some());

        let state = has_state.then_some(
            self.iter()
                .map(|square| match square {
                    Some(cell) => cell.entry().clone().unwrap_or(EMPTY_SQUARE.to_string()),
                    _ => EMPTY_SQUARE.to_string(),
                })
                .collect::<Vec<_>>(),
        );

        // Styles
        let has_styles = self.iter_fills().any(|cell| !cell.style().is_empty());

        let styles = has_styles.then_some(
            self.iter()
                .map(|square| match square {
                    Some(cell) => cell.style(),
                    _ => CellStyle::EMPTY,
                })
                .collect::<Vec<_>>(),
        );

        SquaresData {
            solution,
            state,
            styles,
        }
    }
}

impl ops::Index<Position> for Crossword {
    type Output = Square;

    /// Index the puzzle to retrieve a reference to the [square](Square) at the given [position](Position).
    /// ```
    /// use puzzled_cross::{crossword, Position, Cell};
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
    /// puzzle[pos] = Some(Cell::letter('B'));
    /// assert_eq!(puzzle, puzzle2);
    /// ```
    /// # Panics
    /// Panics if the given `pos` is out of bounds, i.e. `pos.row >= puzzle.rows() || pos.col >= puzzle.cols()`.
    /// ```should_panic
    /// use puzzled_cross::{crossword, Position, Cell};
    ///
    /// let mut puzzle = crossword! (
    ///    [A .]
    ///    [C D]
    /// );
    ///
    /// let pos = Position::new(2, 1);
    /// puzzle[pos] = Some(Cell::letter('E'));
    /// ```
    fn index(&self, pos: Position) -> &Self::Output {
        &self.squares[pos]
    }
}

impl ops::IndexMut<Position> for Crossword {
    /// Index the puzzle to retrieve a mutable reference to the [square](Square) at the given [position](Position).
    /// ```
    /// use puzzled_cross::{crossword, Position, Cell};
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
    /// puzzle[pos] = Some(Cell::letter('B'));
    /// assert_eq!(puzzle, puzzle2);
    /// ```
    /// # Panics
    /// Panics if the given `pos` is out of bounds, i.e. `pos.row >= puzzle.rows() || pos.col >= puzzle.cols()`.
    /// ```should_panic
    /// use puzzled_cross::{crossword, Position, Cell};
    ///
    /// let mut puzzle = crossword! (
    ///    [A .]
    ///    [C D]
    /// );
    ///
    /// let pos = Position::new(2, 1);
    /// puzzle[pos] = Some(Cell::letter('E'));
    /// ```
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.squares[pos]
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct SquaresData {
    pub(crate) solution: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) state: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) styles: Option<Vec<crate::CellStyle>>,
}
