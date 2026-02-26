use puzzled_core::{Cell, CellStyle, Entry, Grid, Square};

use crate::puz::{MISSING_ENTRY_CHAR, NON_PLAYABLE_CHAR};

// State
pub trait WriteStateGrid<T> {
    fn write_state_grid<F>(&self, f: F) -> Grid<u8>
    where
        F: FnMut(&T) -> u8;
}

impl<T> WriteStateGrid<T> for Grid<Option<T>> {
    fn write_state_grid<F>(&self, mut f: F) -> Grid<u8>
    where
        F: FnMut(&T) -> u8,
    {
        self.map_ref(|square| match square {
            None => MISSING_ENTRY_CHAR as u8,
            Some(solution) => f(solution),
        })
    }
}

impl<T> WriteStateGrid<T> for Grid<Entry<T>> {
    fn write_state_grid<F>(&self, mut f: F) -> Grid<u8>
    where
        F: FnMut(&T) -> u8,
    {
        self.map_ref(|entry| match entry.entry() {
            None => MISSING_ENTRY_CHAR as u8,
            Some(solution) => f(solution),
        })
    }
}

impl<T> WriteStateGrid<T> for Grid<Square<Option<T>>> {
    fn write_state_grid<F>(&self, mut f: F) -> Grid<u8>
    where
        F: FnMut(&T) -> u8,
    {
        self.map_ref(|square| match square.inner() {
            None => NON_PLAYABLE_CHAR as u8,
            Some(solution) => solution
                .as_ref()
                .map(&mut f)
                .unwrap_or(MISSING_ENTRY_CHAR as u8),
        })
    }
}

impl<T> WriteStateGrid<T> for Grid<Square<Entry<T>>> {
    fn write_state_grid<F>(&self, mut f: F) -> Grid<u8>
    where
        F: FnMut(&T) -> u8,
    {
        self.map_ref(|square| match square.inner() {
            None => NON_PLAYABLE_CHAR as u8,
            Some(entry) => match entry.entry() {
                Some(solution) => f(solution),
                _ => MISSING_ENTRY_CHAR as u8,
            },
        })
    }
}

// Styles
pub trait WriteStyleGrid<T, U> {
    fn write_combined_style(&self, other: &Grid<U>) -> Grid<CellStyle>;
}

impl<T> WriteStyleGrid<Cell<T>, Entry<T>> for Grid<Cell<T>> {
    fn write_combined_style(&self, entries: &Grid<Entry<T>>) -> Grid<CellStyle> {
        let styles: Vec<_> = self
            .iter()
            .zip(entries.iter())
            .map(|(cell, entry)| cell.style | entry.style())
            .collect();

        Grid::from_vec(styles, self.cols())
            .expect("Constructing GEXT from valid squares and entries")
    }
}

impl<T> WriteStyleGrid<Square<Cell<T>>, Square<Entry<T>>> for Grid<Square<Cell<T>>> {
    fn write_combined_style(&self, entries: &Grid<Square<Entry<T>>>) -> Grid<CellStyle> {
        let styles: Vec<_> = self
            .iter()
            .zip(entries.iter())
            .map(|(puzzle_square, entry_square)| {
                let puzzle_style = puzzle_square
                    .inner()
                    .as_ref()
                    .map(|sq| sq.style)
                    .unwrap_or_default();

                let user_style = entry_square
                    .inner()
                    .as_ref()
                    .map(|sq| sq.style())
                    .unwrap_or_default();

                puzzle_style | user_style
            })
            .collect();

        Grid::from_vec(styles, self.cols())
            .expect("Constructing GEXT from valid squares and entries")
    }
}
