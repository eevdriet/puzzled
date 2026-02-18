use crate::{
    Puzzle, SizeCheck, Square,
    io::{format, windows_1252_to_char},
};
use puzzled_core::Grid;

pub(crate) const NON_PLAYABLE_CELL: u8 = b'.';
pub(crate) const MISSING_ENTRY_CELL: u8 = b'-';

#[derive(Debug)]
pub(crate) struct Grids {
    pub solution: Grid<u8>,
    pub state: Grid<u8>,

    pub width: u8,
    pub height: u8,
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum GridsError {
    #[error(
        "({rows}R, {cols}C) is too large to represent a crossword grid. Make sure rows, cols <= {}",
        u8::MAX
    )]
    Oversized { rows: usize, cols: usize },

    #[error("Row {row} in the grid has an invalid width of {found} (expected {expected})")]
    InvalidWidth { row: u8, found: u8, expected: u8 },

    #[error("The grid has an invalid height of {found} (expected {expected})")]
    InvalidHeight { found: u8, expected: u8 },

    #[error(
        "The grid has invalid dimensions ({rows} rows and {cols} columns). Make sure the size divides the number of columns"
    )]
    InvalidDimensions { cols: u8, rows: u8 },

    #[error("Row {row} has an invalid format: {reason}")]
    InvalidRow { row: u8, reason: String },

    #[error(
        "The solution grid has square '{solution_square}' at {row}R{col}C, while the state grid has '{state_square}' at that position"
    )]
    CellMismatch {
        solution_square: char,
        state_square: char,
        row: u8,
        col: u8,
    },
}

impl SizeCheck for Grid<Square> {
    fn check_size(&self) -> format::Result<()> {
        let rows = self.rows();
        let cols = self.cols();
        let max_size = u8::MAX as usize;

        if cols > max_size || rows > max_size {
            return Err(format::Error::Grids(GridsError::Oversized { rows, cols }));
        }

        Ok(())
    }
}

impl Grids {
    pub(crate) fn from_puzzle(puzzle: &Puzzle) -> format::Result<Self> {
        puzzle.squares().check_size()?;

        let rows = puzzle.rows();
        let cols = puzzle.cols();

        let solution = {
            let mut bytes = Vec::new();

            for square in puzzle.iter() {
                let byte = match square {
                    Square::Black => NON_PLAYABLE_CELL,
                    Square::White(cell) => {
                        cell.solution();
                        4
                    }
                };

                bytes.push(byte);
            }

            Grid::from_vec(bytes, cols).expect("Read correct length")
        };

        let state = {
            let mut bytes = Vec::new();

            for square in puzzle.iter() {
                let byte = match square {
                    Square::Black => NON_PLAYABLE_CELL,
                    Square::White(cell) => match cell.entry() {
                        Some(v) => v.chars().next().unwrap_or(MISSING_ENTRY_CELL as char) as u8,
                        None => MISSING_ENTRY_CELL,
                    },
                };

                bytes.push(byte);
            }

            Grid::from_vec(bytes, cols).expect("Read correct length")
        };

        let grids = Grids {
            solution,
            state,
            width: cols as u8,
            height: rows as u8,
        };
        grids.validate()?;

        Ok(grids)
    }

    fn validate(&self) -> format::Result<()> {
        let grids = [(&self.state, "puzzle"), (&self.solution, "answer")];

        let err = |kind: GridsError| format::Error::Grids(kind);

        for (grid, _) in &grids {
            let len = grid.rows() as u8;

            if len != self.height {
                return Err(err(GridsError::InvalidHeight {
                    found: len,
                    expected: self.height,
                }));
            }

            // Check whether the rows have the correct width
            for (r, row) in grid.iter_rows().enumerate() {
                let len = row.count() as u8;

                if len != self.width {
                    return Err(err(GridsError::InvalidWidth {
                        row: r as u8,
                        found: len,
                        expected: self.width,
                    }));
                }
            }
        }

        // Check that non-playable squares match in the layout and state
        for ((pos, &solution_square), &state_square) in
            self.solution.indexed_iter().zip(self.state.iter())
        {
            if (solution_square == NON_PLAYABLE_CELL) != (state_square == NON_PLAYABLE_CELL) {
                return Err(err(GridsError::CellMismatch {
                    solution_square: windows_1252_to_char(solution_square),
                    state_square: windows_1252_to_char(state_square),
                    row: pos.row as u8,
                    col: pos.col as u8,
                }));
            }
        }

        Ok(())
    }
}
