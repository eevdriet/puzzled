use puzzled_core::{Cell, Entry, Grid, MISSING_ENTRY_CHAR, NON_PLAYABLE_CHAR, Square};

use crate::{
    CellEntries, Context, SquareEntries,
    puz::{
        Extras, Grids,
        read::{self},
        windows_1252_to_char,
    },
};

pub fn read_cell_entries<T, F>(
    grids: &Grids,
    extras: &Extras,
    mut cell_fn: F,
) -> read::Result<CellEntries<T>>
where
    F: FnMut(char) -> read::Result<T>,
{
    grids.validate().context("Cell and entry grids")?;
    let cols = grids.width as usize;

    let mut cells = Vec::with_capacity(cols);
    let mut entries = Vec::with_capacity(cols);

    for ((pos, &solution), &state) in grids.solution.iter_indexed().zip(grids.state.iter()) {
        let style = extras.get_style(pos);

        let cell = match windows_1252_to_char(solution) {
            MISSING_ENTRY_CHAR => None,
            char => Some(cell_fn(char)?),
        };
        cells.push(Cell::new_with_style(cell, style));

        let entry = match windows_1252_to_char(state) {
            MISSING_ENTRY_CHAR => None,
            char => Some(cell_fn(char)?),
        };
        entries.push(Entry::new_with_style(entry, style));
    }

    let cells = Grid::from_vec(cells, cols).expect("Valid cell grid");
    let entries = Grid::from_vec(entries, cols).expect("Valid entry grid");

    Ok((cells, entries))
}

pub fn read_square_entries<T, F>(
    grids: &Grids,
    extras: &Extras,
    mut cell_fn: F,
) -> read::Result<SquareEntries<T>>
where
    F: FnMut(char) -> read::Result<T>,
{
    grids.validate().context("Square and entry grids")?;
    let cols = grids.width as usize;

    let mut squares = Vec::with_capacity(cols);
    let mut entries = Vec::with_capacity(cols);

    for ((pos, &solution), &state) in grids.solution.iter_indexed().zip(grids.state.iter()) {
        let style = extras.get_style(pos);

        let square = match windows_1252_to_char(solution) {
            NON_PLAYABLE_CHAR => Square::new_empty(),
            char => {
                let solution = match char {
                    MISSING_ENTRY_CHAR => None,
                    _ => Some(cell_fn(char)?),
                };

                let cell = Cell::new_with_style(solution, style);
                Square::new(cell)
            }
        };
        squares.push(square);

        let entry = match windows_1252_to_char(state) {
            NON_PLAYABLE_CHAR => Square::new_empty(),
            char => {
                let solution = match char {
                    MISSING_ENTRY_CHAR => None,
                    _ => Some(cell_fn(char)?),
                };

                let entry = Entry::new_with_style(solution, style);
                Square::new(entry)
            }
        };
        entries.push(entry);
    }

    let cells = Grid::from_vec(squares, cols).expect("Valid square grid");
    let entries = Grid::from_vec(entries, cols).expect("Valid entry grid");

    Ok((cells, entries))
}
