use std::str::FromStr;

use puzzled_core::{Grid, GridError};

use crate::{
    format,
    puz::read::{CellEntries, SquareEntries},
    text::read::{self, TxtState},
};

impl<'a> TxtState<'a> {
    pub fn read_cells_and_entries<T>(&mut self) -> read::Result<CellEntries<T>>
    where
        T: FromStr,
    {
        self.read_grids2_with(|row| row.parse_cell_entry::<T>())
    }

    pub fn read_squares_and_entries<T>(&mut self) -> read::Result<SquareEntries<T>>
    where
        T: FromStr,
    {
        self.read_grids2_with(|row| row.parse_square_entry::<T>())
    }

    fn read_grids2_with<A, B, F>(&mut self, mut parse_entry: F) -> read::Result<(Grid<A>, Grid<B>)>
    where
        F: FnMut(&mut TxtState<'_>) -> Option<(A, B)>,
    {
        let err = |err: GridError| format::Error::Grid(err);

        let mut cells = Vec::new();
        let mut entries = Vec::new();

        let mut cols = None;
        let mut rows = 0;

        while let Some(line) = self.next_delimited("[", "]") {
            rows += 1;

            let mut row_width = 0;
            let mut row_parser = TxtState::new(line, self.strict);

            while let Some((cell, entry)) = parse_entry(&mut row_parser) {
                row_width += 1;

                cells.push(cell);
                entries.push(entry);
            }

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

        let cols = cols.ok_or(format::Error::Grid(GridError::InvalidDimensions {
            rows,
            cols: 0,
        }))?;

        let cells = Grid::from_vec(cells, cols as usize).expect("Read cell grid correctly");
        let entries = Grid::from_vec(entries, cols as usize).expect("Read entries grid correctly");

        Ok((cells, entries))
    }
}
