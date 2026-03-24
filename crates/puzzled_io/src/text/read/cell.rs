use chumsky::{
    IterParser, Parser,
    extra::Err,
    prelude::{group, just, one_of},
};
use puzzled_core::{Cell, CellStyle, Entry, Grid, MISSING_ENTRY_CHAR};

use crate::text::read::{ParseError, grid};

pub fn cell_entry<'a, T, P>(
    value: P,
) -> impl Parser<'a, &'a str, (Cell<T>, Entry<T>), Err<ParseError<'a>>> + Clone
where
    P: Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone,
{
    group((
        solution(value.clone()).padded(),
        cell_style().or_not().padded(),
        entry(value.clone()).padded(),
    ))
    .padded()
    .map(|(solution, opt_style, entry)| {
        let mut style = opt_style.unwrap_or_default();
        if solution.is_some() {
            style |= CellStyle::INITIALLY_REVEALED;
        }

        let cell = Cell::new_with_style(solution, style);
        let entry = Entry::new_with_style(entry, style);

        (cell, entry)
    })
}

pub fn cell_style<'a>() -> impl Parser<'a, &'a str, CellStyle, Err<ParseError<'a>>> + Clone {
    one_of("*@~!")
        .repeated()
        .fold(CellStyle::default(), |style, marker| match marker {
            '*' => style | CellStyle::REVEALED,
            '!' => style | CellStyle::INCORRECT,
            '~' => style | CellStyle::PREVIOUSLY_INCORRECT,
            '@' => style | CellStyle::CIRCLED,
            _ => unreachable!("Only parsed one_of(\"*@~!\")"),
        })
}

pub fn cell_entry_grids<'a, T, P>(
    value: P,
) -> impl Parser<'a, &'a str, (Grid<Cell<T>>, Grid<Entry<T>>), Err<ParseError<'a>>>
where
    P: Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone,
{
    grid(cell_entry(value)).map(|cell_entries| {
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

fn solution<'a, T, P>(value: P) -> impl Parser<'a, &'a str, Option<T>, Err<ParseError<'a>>> + Clone
where
    P: Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone,
{
    just(MISSING_ENTRY_CHAR).map(|_| None).or(value.map(Some))
}

fn entry<'a, T, P>(value: P) -> impl Parser<'a, &'a str, Option<T>, Err<ParseError<'a>>> + Clone
where
    P: Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone,
{
    value.delimited_by(just('('), just(')')).or_not()
}

#[cfg(test)]
mod tests {
    use chumsky::text;
    use puzzled_core::CellStyle;
    use rstest::rstest;

    use super::*;

    const _E: CellStyle = CellStyle::empty();
    const _I: CellStyle = CellStyle::INCORRECT;
    const _P: CellStyle = CellStyle::PREVIOUSLY_INCORRECT;
    const _R: CellStyle = CellStyle::REVEALED;
    const _C: CellStyle = CellStyle::CIRCLED;

    #[rstest]
    #[case("-", None, None, _E)]
    #[case("10", Some(10), None, _E)]
    #[case("10*", Some(10), None, _R)]
    #[case("10*@", Some(10), None, _R | _C)]
    #[case("10*@ (10)", Some(10), Some(10), _R | _C)]
    #[case("10*@ (22)", Some(10), Some(22), _R | _C)]
    // #[case("10 10", Some(10), None, _R | _C)]
    // #[case("10*@ 10", Some(10), None, _R | _C)]
    // zfZTQFQ3h9SL98BK
    fn test_cell_entry(
        #[case] input: &str,
        #[case] cell_val: Option<usize>,
        #[case] entry_val: Option<usize>,
        #[case] style: CellStyle,
    ) {
        let value = text::digits::<_, Err<ParseError<'_>>>(10)
            .to_slice()
            .from_str()
            .unwrapped();

        let (cell, entry) = cell_entry(value)
            .parse(input)
            .into_output()
            .expect("Parsing should succeed");

        assert_eq!(cell_val, cell.solution);
        assert_eq!(style, cell.style);
        assert_eq!(entry_val.as_ref(), entry.entry());
    }
}
