use crate::{Direction, Grid, Offset, Parser, Position, Result};

const NON_PLAYABLE_CELL: u8 = b'-';

#[derive(Debug)]
pub(crate) struct PuzzleGrid<'a> {
    pub solution: Grid<u8>,
    pub state: Grid<u8>,

    pub solution_region: &'a [u8],
    pub state_region: &'a [u8],
}

impl PuzzleGrid<'_> {
    pub fn starts_across(&self, pos: Position) -> bool {
        self.is_playable(pos)
            && !self.is_playable(pos + Offset::UP)
            && !self.is_playable(pos + Offset::DOWN)
    }

    pub fn starts_down(&self, pos: Position) -> bool {
        self.is_playable(pos)
            && !self.is_playable(pos + Offset::LEFT)
            && !self.is_playable(pos + Offset::RIGHT)
    }

    fn is_playable(&self, pos: Position) -> bool {
        let Some(&cell) = self.solution.get(pos) else {
            return false;
        };

        cell != NON_PLAYABLE_CELL
    }

    pub fn find_playable_len(&self, pos: Position, dir: Direction) -> u8 {
        let offset = match dir {
            Direction::Across => Offset::RIGHT,
            Direction::Down => Offset::DOWN,
        };

        let count = (0..)
            .scan(pos, |acc, _| {
                *acc += offset;
                self.is_playable(*acc).then_some(*acc)
            })
            .count() as u8;

        count + 1
    }
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
        other_cell: u8,
        row: u8,
        col: u8,
    },
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_grid(&mut self, width: u8, height: u8) -> Result<PuzzleGrid<'a>> {
        // Determine the puzzle size from its width and height
        let size = usize::from(width) * usize::from(height);

        // Parse twice the puzzle size, for both the layout (solution) and state (partial user solution)
        // Then convert both to a 2D grid
        let solution_region = self.read(size, "Puzzle solution")?;
        let solution =
            Grid::new(solution_region.into(), width).expect("Read correct length region");

        let state_region = self.read(size, "Player state")?;
        let state = Grid::new(state_region.into(), width).expect("Read correct length region");

        // Create the puzzle and check its validity
        let puzzle = PuzzleGrid {
            solution,
            solution_region,
            state,
            state_region,
        };
        validate_puzzle(puzzle, width, height)
    }
}

fn validate_puzzle(puzzle: PuzzleGrid, width: u8, height: u8) -> Result<PuzzleGrid> {
    let grids = [(&puzzle.state, "puzzle"), (&puzzle.solution, "answer")];

    for (grid, name) in &grids {
        let len = grid.rows();

        if len != height {
            return Err(GridError::InvalidHeight {
                grid: name.to_string(),
                found: len,
                expected: height,
            }
            .into());
        }

        // Check whether the rows have the correct width
        for (r, row) in grid.iter_rows().enumerate() {
            let len = row.count() as u8;

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
    for ((layout_pos, &layout_cell), (state_pos, &state_cell)) in
        puzzle.state.iter().zip(puzzle.solution.iter())
    {
        if layout_cell == NON_PLAYABLE_CELL && state_cell != NON_PLAYABLE_CELL {
            return Err(GridError::CellMismatch {
                grid_non_playable: grids[0].1.to_string(),
                grid_other: grids[1].1.to_string(),
                other_cell: state_cell,
                row: layout_pos.row,
                col: layout_pos.col,
            }
            .into());
        }

        if state_cell == NON_PLAYABLE_CELL && layout_cell != NON_PLAYABLE_CELL {
            return Err(GridError::CellMismatch {
                grid_non_playable: grids[1].1.to_string(),
                grid_other: grids[0].1.to_string(),
                other_cell: layout_cell,
                row: state_pos.row,
                col: state_pos.col,
            }
            .into());
        }
    }

    Ok(puzzle)
}
