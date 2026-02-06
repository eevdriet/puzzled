use std::borrow::Cow;

use crate::{Parser, Region, Result, parse_str};

const NON_PLAYABLE_CELL: char = '-';
const BLANK_CELL: char = '.';

#[derive(Debug)]
pub(crate) struct Grid<'a> {
    pub solution: Vec<Cow<'a, str>>,
    pub state: Vec<Cow<'a, str>>,

    pub solution_region: Region<'a>,
    pub state_region: Region<'a>,
}

#[derive(Debug, thiserror::Error)]
pub enum GridError {
    #[error("Row {row} in the {grid} has an invalid width of {found} (expected {expected})")]
    InvalidWidth {
        grid: String,
        row: u8,
        found: u8,
        expected: u8,
    },

    #[error("The {grid} has an invalid height of {found} (expected {expected})")]
    InvalidHeight {
        grid: String,
        found: u8,
        expected: u8,
    },

    #[error(
        "The {grid_non_playable} has a non-playable cell at {row}R{col}C, while the {grid_other} has {other_cell} at that position"
    )]
    CellMismatch {
        grid_non_playable: String,
        grid_other: String,
        other_cell: char,
        row: u8,
        col: u8,
    },
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_grid(&mut self, width: u8, height: u8) -> Result<Grid<'a>> {
        // Determine the puzzle size from its width and height
        let size = usize::from(width) * usize::from(height);

        // Parse twice the puzzle size, for both the layout (solution) and state (partial user solution)
        // Then convert both to a 2D grid
        let (solution, solution_region) = self.read_region(|p| p.read(size, "Puzzle solution"))?;
        let solution = parse_grid(&solution, width);

        let (state, state_region) = self.read_region(|p| p.read(size, "Player state"))?;
        let state = parse_grid(&state, width);

        // Create the puzzle and check its validity
        let puzzle = Grid {
            solution,
            solution_region,
            state,
            state_region,
        };
        validate_puzzle(puzzle, width, height)
    }
}

fn parse_grid<'a>(bytes: &'a [u8], width: u8) -> Vec<Cow<'_, str>> {
    bytes.chunks(width as usize).map(parse_str).collect()
}

fn validate_puzzle(puzzle: Grid, width: u8, height: u8) -> Result<Grid> {
    let grids = [(&puzzle.state, "puzzle"), (&puzzle.solution, "answer")];

    for (grid, name) in &grids {
        // Check whether the height of the puzzle is valid
        let len = grid.len() as u8;

        if len != height {
            return Err(GridError::InvalidHeight {
                grid: name.to_string(),
                found: len,
                expected: height,
            }
            .into());
        }

        // Check whether the rows have the correct width
        for (r, row) in grid.iter().enumerate() {
            let len = row.len() as u8;

            if len != width {
                return Err(GridError::InvalidWidth {
                    grid: name.to_string(),
                    row: r as u8,
                    found: len,
                    expected: width,
                }
                .into());
            }
        }
    }

    // Check that non-playable cells match in the layout and state
    for (r, (layout_row, state_row)) in puzzle.state.iter().zip(puzzle.solution.iter()).enumerate()
    {
        let r = r as u8;

        for (c, (layout_cell, state_cell)) in layout_row.chars().zip(state_row.chars()).enumerate()
        {
            let c = c as u8;

            if layout_cell == NON_PLAYABLE_CELL && state_cell != NON_PLAYABLE_CELL {
                return Err(GridError::CellMismatch {
                    grid_non_playable: grids[0].1.to_string(),
                    grid_other: grids[1].1.to_string(),
                    other_cell: state_cell,
                    row: r,
                    col: c,
                }
                .into());
            }

            if state_cell == NON_PLAYABLE_CELL && layout_cell != NON_PLAYABLE_CELL {
                return Err(GridError::CellMismatch {
                    grid_non_playable: grids[1].1.to_string(),
                    grid_other: grids[0].1.to_string(),
                    other_cell: layout_cell,
                    row: r,
                    col: c,
                }
                .into());
            }
        }
    }

    Ok(puzzle)
}
