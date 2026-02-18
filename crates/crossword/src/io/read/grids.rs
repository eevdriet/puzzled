use puzzled_core::Grid;

use crate::Square;
use crate::io::{
    Context, Grids, GridsError, PuzRead, SECTION_SEPARATOR, TxtReader, TxtState, format, read,
};

impl Grids {
    pub(crate) fn read_from<R: PuzRead>(
        reader: &mut R,
        width: u8,
        height: u8,
    ) -> read::Result<Self> {
        let size = usize::from(width) * usize::from(height);

        let solution = reader.read_vec(size).context("Solution grid")?;
        let solution = Grid::new(solution, width).expect("Read correct length");

        let state = reader.read_vec(size).context("State grid")?;
        let state = Grid::new(state, width).expect("Read correct length");

        Ok(Self {
            solution,
            state,
            width,
            height,
        })
    }
}

impl<'a> TxtReader {
    pub(crate) fn parse_grid(&self, state: &mut TxtState<'a>) -> read::Result<Grid<Square>> {
        let mut squares = Vec::new();
        let context = "Puzzle grid";

        let err = |err: GridsError| format::Error::Grids(err);

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
                    return Err(err(GridsError::InvalidWidth {
                        row: rows,
                        found: row_width,
                        expected: width,
                    }))
                    .context(context.to_string());
                }
            } else {
                cols = Some(row_width);
            }
        }

        let cols = cols
            .ok_or(err(GridsError::InvalidDimensions { rows, cols: 0 }))
            .context(context.to_string())?;

        let grid = Grid::new(squares, cols)
            .ok_or(err(GridsError::InvalidDimensions { rows, cols }))
            .context(context.to_string())?;

        Ok(grid)
    }

    fn parse_row(row: u8, line: &str) -> read::Result<Vec<Square>> {
        if !line.starts_with('[') || !line.ends_with(']') {
            return Err(format::Error::Grids(GridsError::InvalidRow {
                row,
                reason: "Should be delimited by [...]".to_string(),
            }))
            .context("Puzzle grid");
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
