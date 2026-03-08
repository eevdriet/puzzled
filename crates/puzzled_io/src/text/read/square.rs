use chumsky::{Parser, extra::Err, prelude::just};
use puzzled_core::{Cell, Entry, Grid, NON_PLAYABLE_CHAR, Square};

use crate::text::read::{ParseError, cell_entry, grid};

pub fn square_entry<'a, T, P>(
    value: P,
) -> impl Parser<'a, &'a str, (Square<Cell<T>>, Square<Entry<T>>), Err<ParseError<'a>>> + Clone
where
    P: Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone,
{
    just(NON_PLAYABLE_CHAR)
        .map(|_| (Square::new_empty(), Square::new_empty()))
        .or(cell_entry(value).map(|(cell, entry)| (Square::new(cell), Square::new(entry))))
}

pub fn square_entry_grids<'a, T, P>(
    value: P,
) -> impl Parser<'a, &'a str, (Grid<Square<Cell<T>>>, Grid<Square<Entry<T>>>), Err<ParseError<'a>>>
where
    P: Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone,
{
    grid(square_entry(value)).map(|cell_entries| {
        let cols = cell_entries.cols();

        let (cells, entries) = cell_entries.into_iter().fold(
            (vec![], vec![]),
            |(mut cells, mut entries), (cell, entry)| {
                cells.push(cell);
                entries.push(entry);

                (cells, entries)
            },
        );

        let cells = Grid::from_vec(cells, cols).expect("Read cells from grid");
        let entries = Grid::from_vec(entries, cols).expect("Read entries from grid");

        (cells, entries)
    })
}
