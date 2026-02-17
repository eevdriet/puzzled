use crate::{
    Grid, Puzzle, Square,
    io::{format, windows_1252_to_char},
};

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

impl Grids {
    pub(crate) fn from_puzzle(puzzle: &Puzzle) -> format::Result<Self> {
        let width = puzzle.cols();
        let height = puzzle.rows();

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

            Grid::new(bytes, width).expect("Read correct length")
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

            Grid::new(bytes, width).expect("Read correct length")
        };

        let grids = Grids {
            solution,
            state,
            width,
            height,
        };
        grids.validate()?;

        Ok(grids)
    }

    fn validate(&self) -> format::Result<()> {
        let grids = [(&self.state, "puzzle"), (&self.solution, "answer")];

        let err = |kind: GridsError| format::Error::Grids(kind);

        for (grid, _) in &grids {
            let len = grid.rows();

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
        for ((&solution_square, &state_square), pos) in self
            .solution
            .iter()
            .zip(self.state.iter())
            .zip(self.solution.positions())
        {
            if (solution_square == NON_PLAYABLE_CELL) != (state_square == NON_PLAYABLE_CELL) {
                return Err(err(GridsError::CellMismatch {
                    solution_square: windows_1252_to_char(solution_square),
                    state_square: windows_1252_to_char(state_square),
                    row: pos.row,
                    col: pos.col,
                }));
            }
        }

        Ok(())
    }
}
