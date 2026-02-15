use thiserror::Error;

use crate::io::{
    Error, ErrorKind, PuzParser, PuzState, Result, SECTION_SEPARATOR, Span, TxtParser, TxtState,
    windows_1252_to_char,
};
use crate::{Grid, Square};

pub(crate) const NON_PLAYABLE_CELL: u8 = b'.';

#[derive(Debug)]
pub(crate) struct PuzzleGrid {
    pub solution: Grid<u8>,
    pub state: Grid<u8>,

    pub solution_span: Span,
    pub state_span: Span,
}

#[derive(Debug, Error, Clone)]

pub enum GridError {
    #[error("Row {row} in the grid has an invalid width of {found} (expected {expected})")]
    InvalidWidth { row: u8, found: u8, expected: u8 },

    #[error("The grid has an invalid height of {found} (expected {expected})")]
    InvalidHeight { found: u8, expected: u8 },

    #[error(
        "The grid has invalid dimensions ({rows} rows and {cols} columns). Make sure the size divides the number of columns"
    )]
    InvalidDimensions { cols: u8, rows: u8 },

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

impl<'a> PuzParser {
    pub(crate) fn parse_grid(
        &self,
        width: u8,
        height: u8,
        state: &mut PuzState<'a>,
    ) -> Result<PuzzleGrid> {
        // Determine the puzzle size from its width and height
        let size = usize::from(width) * usize::from(height);

        // Parse twice the puzzle size, for both the layout (solution) and state (partial user solution)
        // Then convert both to a 2D grid
        let (solution, solution_span) = state.read_span(|s| s.read(size, "Puzzle solution"))?;
        let solution = Grid::new(solution.to_vec(), width).expect("Read correct length region");

        let (state, state_span) = state.read_span(|s| s.read(size, "Player state"))?;
        let state = Grid::new(state.into(), width).expect("Read correct length region");

        // Create the puzzle and check its validity
        let puzzle = PuzzleGrid {
            solution,
            solution_span,
            state,
            state_span,
        };

        Self::validate_puzzle(puzzle, width, height)
    }

    fn validate_puzzle(puzzle: PuzzleGrid, width: u8, height: u8) -> Result<PuzzleGrid> {
        let grids = [(&puzzle.state, "puzzle"), (&puzzle.solution, "answer")];

        for (grid, name) in &grids {
            let len = grid.rows();

            if len != height {
                return Err(Error {
                    span: 0..0,
                    context: name.to_string(),
                    kind: GridError::InvalidHeight {
                        found: len,
                        expected: height,
                    }
                    .into(),
                });
            }

            // Check whether the rows have the correct width
            for (r, row) in grid.iter_rows().enumerate() {
                let len = row.count() as u8;

                if len != width {
                    return Err(Error {
                        span: 0..0,
                        context: name.to_string(),
                        kind: GridError::InvalidWidth {
                            row: r as u8,
                            found: len,
                            expected: width,
                        }
                        .into(),
                    });
                }
            }
        }

        // Check that non-playable squares match in the layout and state
        for ((&solution_square, &state_square), pos) in puzzle
            .solution
            .iter()
            .zip(puzzle.state.iter())
            .zip(puzzle.solution.positions())
        {
            if (solution_square == NON_PLAYABLE_CELL) != (state_square == NON_PLAYABLE_CELL) {
                return Err(Error {
                    span: puzzle.solution_span,
                    kind: GridError::CellMismatch {
                        solution_square: windows_1252_to_char(solution_square),
                        state_square: windows_1252_to_char(state_square),
                        row: pos.row,
                        col: pos.col,
                    }
                    .into(),
                    context: "Puzzle grids".to_string(),
                });
            }
        }

        Ok(puzzle)
    }
}

impl<'a> TxtParser {
    pub(crate) fn parse_grid(&self, state: &mut TxtState<'a>) -> Result<Grid<Square>> {
        let mut squares = Vec::new();
        let context = "Puzzle grid".to_string();

        let mut cols = None;
        let mut rows = 0;

        while let Some(line) = state.next() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                continue;
            }
            if line == SECTION_SEPARATOR {
                break;
            }

            rows += 1;

            // Parse the next row and verify its width
            let row = Self::parse_row(rows, line)?;
            let row_width = row.len() as u8;
            squares.extend(row);

            if let Some(width) = cols {
                if width != row_width {
                    return Err(Error {
                        span: 0..0,
                        kind: GridError::InvalidWidth {
                            row: rows,
                            found: row_width,
                            expected: width,
                        }
                        .into(),
                        context,
                    });
                }
            } else {
                cols = Some(row_width);
            }
        }

        let cols = cols.ok_or(Error {
            span: 0..0,
            kind: GridError::InvalidDimensions { rows, cols: 0 }.into(),
            context: context.clone(),
        })?;

        let grid = Grid::new(squares, cols).ok_or(Error {
            span: 0..0,
            kind: GridError::InvalidDimensions { rows, cols }.into(),
            context,
        })?;

        Ok(grid)
    }

    fn parse_row(row: u8, line: &str) -> Result<Vec<Square>> {
        let context = format!("Row #{row}");

        if !line.starts_with('[') || !line.ends_with(']') {
            return Err(Error {
                span: 0..0,
                kind: ErrorKind::Custom("[...]".to_string()),
                context,
            });
        }

        let line = &line[1..line.len() - 1];
        let mut squares = Vec::new();

        for token in line.split_whitespace() {
            let square = match token {
                "." => Square::Black,
                word if word.len() == 1 => {
                    Square::letter(word.chars().next().expect("Word is not empty"))
                }
                rebus => Square::rebus(rebus.to_string()),
            };

            squares.push(square);
        }

        Ok(squares)
    }
}
