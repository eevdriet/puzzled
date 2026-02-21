use crate::{Context, PuzRead, PuzWrite, format, read, windows_1252_to_char, write};
use puzzled_core::Grid;

pub const NON_PLAYABLE_CELL: char = '.';
pub const MISSING_ENTRY_CELL: char = '-';

#[doc(hidden)]
#[derive(Debug)]
pub struct Grids {
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
    pub fn validate(&self) -> format::Result<()> {
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
            self.solution.iter_indexed().zip(self.state.iter())
        {
            if (solution_square == NON_PLAYABLE_CELL as u8)
                != (state_square == NON_PLAYABLE_CELL as u8)
            {
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

/// # Read
impl Grids {
    pub(crate) fn read_from<R>(reader: &mut R, width: u8, height: u8) -> read::Result<Self>
    where
        R: PuzRead,
    {
        let uwidth = width as usize;
        let size = uwidth * usize::from(height);

        let solution = reader.read_vec(size).context("Solution grid")?;
        let solution = Grid::from_vec(solution, uwidth).expect("Read correct length");

        let state = reader.read_vec(size).context("State grid")?;
        let state = Grid::from_vec(state, uwidth).expect("Read correct length");

        Ok(Self {
            solution,
            state,
            width,
            height,
        })
    }
}

/// # Write
impl Grids {
    pub(crate) fn write_with<W: PuzWrite>(&self, writer: &mut W) -> write::Result<()> {
        writer
            .write_all(self.solution.data())
            .context("Solution grid")?;

        writer.write_all(self.state.data()).context("State grid")?;

        Ok(())
    }
}
