use std::{fmt::Debug, str::FromStr};

use puzzled_core::{Grid, GridError};

use crate::{
    CellEntries, SquareEntries, format,
    text::read::{self, TxtState},
};

impl<'a> TxtState<'a> {
    pub fn read_cells_and_entries<T>(&mut self) -> read::Result<CellEntries<T>>
    where
        T: FromStr + Debug,
    {
        self.read_grids2_with(|row| row.parse_cell_entry::<T>())
    }

    pub fn read_squares_and_entries<T>(&mut self) -> read::Result<SquareEntries<T>>
    where
        T: FromStr + Debug,
    {
        self.read_grids2_with(|row| row.parse_square_entry::<T>())
    }

    fn read_grids2_with<A, B, F>(&mut self, mut parse_entry: F) -> read::Result<(Grid<A>, Grid<B>)>
    where
        F: FnMut(&mut TxtState<'_>) -> Option<(A, B)>,
        A: Debug,
        B: Debug,
    {
        let err = |err: GridError| format::Error::Grid(err);

        let mut cells = Vec::new();
        let mut entries = Vec::new();

        let mut cols = None;
        let mut rows = 0;

        while let Some(line) = self.next_delimited("[", "]") {
            eprintln!("Line: {line}");
            rows += 1;

            let mut row_width = 0;
            let mut row_parser = TxtState::new(line, self.strict);

            while let Some((cell, entry)) = parse_entry(&mut row_parser) {
                row_width += 1;

                cells.push(cell);
                entries.push(entry);
            }

            eprintln!("Cells: {cells:?}");

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

impl<'a> TxtState<'a> {
    pub fn read_sided_cells_and_entries<T>(
        &mut self,
    ) -> read::Result<(CellEntries<T>, [Vec<Option<T>>; 4])>
    where
        T: FromStr + Debug,
    {
        self.read_sided_grids2_with(|row| row.parse_cell_entry::<T>())
    }

    pub fn read_sided_squares_and_entries<T>(
        &mut self,
    ) -> read::Result<(SquareEntries<T>, [Vec<Option<T>>; 4])>
    where
        T: FromStr + Debug,
    {
        self.read_sided_grids2_with(|row| row.parse_square_entry::<T>())
    }

    fn read_sided_grids2_with<A, B, T, F>(
        &mut self,
        mut parse_entry: F,
    ) -> read::Result<((Grid<A>, Grid<B>), [Vec<Option<T>>; 4])>
    where
        F: FnMut(&mut TxtState<'_>) -> Option<(A, B)>,
        T: FromStr + Debug,
    {
        eprintln!("A");
        let err = |err: GridError| format::Error::Grid(err);

        let mut cells = Vec::new();
        let mut entries = Vec::new();

        let mut top = Vec::new();
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut bottom = Vec::new();

        // Parse the top row separately
        eprintln!("B");
        let mut top_parser = TxtState::new(
            self.next_line()
                .ok_or_else(|| read::Error::Custom("Bleh".to_string()))?,
            self.strict,
        );

        while !top_parser.is_eof() {
            let value = top_parser.parse_value();
            top.push(value);
        }

        // Parse the grid and left/right
        let mut cols = None;
        let mut rows = 0;

        while let Some(line) = self.next_delimited("[", "]") {
            eprintln!("Delim line: {line}");
            rows += 1;

            let mut row_width = 0;
            let mut row_parser = TxtState::new(line, self.strict);

            left.push(row_parser.parse_value());

            while let Some((cell, entry)) = parse_entry(&mut row_parser) {
                row_width += 1;

                cells.push(cell);
                entries.push(entry);
            }

            right.push(row_parser.parse_value());

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

        // Parse the bottom row separately
        let mut bottom_parser = TxtState::new(
            self.next_line()
                .ok_or_else(|| read::Error::Custom("Bleh".to_string()))?,
            self.strict,
        );

        while !bottom_parser.is_eof() {
            let value = bottom_parser.parse_value();
            bottom.push(value);
        }

        let cols = cols.ok_or(format::Error::Grid(GridError::InvalidDimensions {
            rows,
            cols: 0,
        }))?;

        let cells = Grid::from_vec(cells, cols as usize).expect("Read cell grid correctly");
        let entries = Grid::from_vec(entries, cols as usize).expect("Read entries grid correctly");

        Ok(((cells, entries), [top, right, bottom, left]))
    }
}
