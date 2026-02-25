use puzzled_core::{Grid, GridError};

use crate::{
    format,
    text::read::{self, TxtState},
};

impl<'a> TxtState<'a> {
    pub fn read_grid<T, F>(&mut self, row_fn: &mut F) -> read::Result<Grid<T>>
    where
        F: FnMut(&'a str) -> Vec<T>,
    {
        let mut grid = Vec::new();

        let err = |err: GridError| format::Error::Grid(err);

        let mut cols = None;
        let mut rows = 0;

        while let Some(line) = self.peek_line() {
            let line = line.trim();

            // Skip empty lines and stop parsing grid at separator
            if line.is_empty() {
                self.next_line();
                continue;
            }

            if !line.starts_with("[") {
                break;
            }

            let line = self.next_line().expect("Already peeked").trim();
            rows += 1;

            // Parse the next row and verify its width
            let row = read_grid_row(rows, line, row_fn)?;
            let row_width = row.len() as u8;
            grid.extend(row);

            if let Some(width) = cols
                && width != row_width
            {
                return Err(err(GridError::InvalidWidth {
                    row: rows,
                    found: row_width,
                    expected: width,
                })
                .into());
            } else {
                cols = Some(row_width);
            }
        }

        let cols = cols.ok_or(err(GridError::InvalidDimensions { rows, cols: 0 }))?;
        let grid = Grid::from_vec(grid, cols as usize).map_err(err)?;

        Ok(grid)
    }
}

fn read_grid_row<'a, T, F>(row: u8, line: &'a str, row_fn: &mut F) -> format::Result<Vec<T>>
where
    F: FnMut(&'a str) -> Vec<T>,
{
    if !line.starts_with('[') || !line.ends_with(']') {
        let err = GridError::InvalidRow {
            row,
            reason: "Should be delimited by [...]".to_string(),
        };

        return Err(format::Error::Grid(err));
    }

    let line = &line[1..line.len() - 1];

    let cells = row_fn(line);
    Ok(cells)
}
