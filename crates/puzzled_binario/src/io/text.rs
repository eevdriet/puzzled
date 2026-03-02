use chumsky::{
    IterParser, Parser,
    extra::Err,
    prelude::{EmptyErr, any},
};
use puzzled_core::{Grid, Metadata, Timer};
use puzzled_io::chumsky::{
    TxtPuzzle,
    read::{cell_entry, grid},
};

use crate::{Binario, BinarioState, Bit};

pub fn bit<'a>() -> impl Parser<'a, &'a str, Bit, Err<EmptyErr>> + Clone {
    any::<&str, Err<EmptyErr>>()
        .repeated()
        .at_least(1)
        .collect::<String>()
        .try_map(|str, _span| {
            let bit = match str.as_str() {
                // Zero
                "0" => Bit::Zero,
                x if x.eq_ignore_ascii_case("false") => Bit::Zero,
                x if x.eq_ignore_ascii_case("f") => Bit::Zero,

                // One
                "1" => Bit::One,
                x if x.eq_ignore_ascii_case("true") => Bit::One,
                x if x.eq_ignore_ascii_case("t") => Bit::One,

                // Error
                _ => return Err(EmptyErr::default()),
            };

            Ok(bit)
        })
}

impl TxtPuzzle<BinarioState> for Binario {
    fn read_text<'a>() -> impl Parser<'a, &'a str, (Self, BinarioState), Err<EmptyErr>> {
        grid(cell_entry(bit())).map(|cell_entries| {
            let cols = cell_entries.cols();
            eprintln!("Cols: {cols}");

            let (cells, solutions, entries) = cell_entries.into_iter().fold(
                (vec![], vec![], vec![]),
                |(mut cells, mut solutions, mut entries), (cell, entry)| {
                    solutions.push(cell.solution);
                    cells.push(cell);
                    entries.push(entry);

                    (cells, solutions, entries)
                },
            );

            let cells = Grid::from_vec(cells, cols).expect("Read cells from grid");
            let solutions = Grid::from_vec(solutions, cols).expect("Read solutions from grid");
            let entries = Grid::from_vec(entries, cols).expect("Read entries from grid");

            let timer = Timer::default();
            let meta = Metadata::default();

            let puzzle = Binario::new(cells, meta);
            let state = BinarioState::new(solutions, entries, timer);

            (puzzle, state)
        })
    }
}

#[cfg(test)]
mod tests {
    use puzzled_io::{ChumskyPuzzle, chumsky::read::grid_row};
    use rstest::rstest;

    use super::*;
    use crate::binario;

    const T: Bit = Bit::One;
    const F: Bit = Bit::Zero;

    #[rstest]
    #[case("0", Some(F))]
    #[case("1", Some(T))]
    #[case("f", Some(F))]
    #[case("F", Some(F))]
    #[case("t", Some(T))]
    #[case("T", Some(T))]
    #[case("false", Some(F))]
    #[case("FAlse", Some(F))]
    fn parse_bit(#[case] input: &str, #[case] expected: Option<Bit>) {
        let bit = bit().parse(input).into_output();

        assert_eq!(expected, bit);
    }

    #[rstest]
    #[case(" 0 ", vec![Some(F)])]
    #[case("[ - ]", vec![None])]
    #[case("[ - - - ]", vec![None, None, None])]
    #[case("[ - 0 ]", vec![None, Some(F)])]
    fn test_row(#[case] input: &str, #[case] expected: Vec<Option<Bit>>) {
        let solutions: Vec<_> = grid_row(cell_entry(bit()))
            .parse(input)
            .into_output()
            .expect("Parsing should succeed")
            .into_iter()
            .map(|(cell, _entry)| cell.solution)
            .collect();

        assert_eq!(solutions, expected);
    }

    #[rstest]
    #[case("[ 0 ]", vec![Some(F)])]
    #[case("[ - ]", vec![None])]
    #[case("[ - - - ]", vec![None, None, None])]
    #[case("[ - 0 ]", vec![None, Some(F)])]
    fn test_row(#[case] input: &str, #[case] expected: Vec<Option<Bit>>) {
        let solutions: Vec<_> = grid_row(cell_entry(bit()))
            .parse(input)
            .into_output()
            .expect("Parsing should succeed")
            .into_iter()
            .map(|(cell, _entry)| cell.solution)
            .collect();

        assert_eq!(solutions, expected);
    }

    #[test]
    fn write() {
        let puzzle = binario!(
                [- - - - - 1 - - 1 0]
                [- - 1 - - - - - - -]
                [- - - 1 - 1 - - 0 0]
                [1 - - 1 1 - - 1 0 -]
                [- 0 - - - - - - - -]
                [- - - - - - - - - -]
                [- 0 0 - 1 - - - - 1]
                [- 0 - - - - 1 0 - -]
                [- - - 0 - - 1 - - 1]
                [0 0 - - - - - - 1 -]
        );

        puzzle
            .save_text("example01")
            .expect("to write binario as text");
    }
}
