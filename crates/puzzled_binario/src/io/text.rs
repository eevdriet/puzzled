use chumsky::{
    Parser,
    extra::Err,
    prelude::{choice, just},
};
use puzzled_core::{Metadata, Timer};
use puzzled_io::text::{
    TxtPuzzle,
    read::{self, ParseError, cell_entry_grids, ignore_case_keyword},
};

use crate::{Binario, BinarioState, Bit};

pub fn bit<'a>() -> impl Parser<'a, &'a str, Bit, Err<ParseError<'a>>> + Clone {
    choice((
        // Zeroes
        just("0").to(Bit::Zero),
        ignore_case_keyword("false").to(Bit::Zero),
        ignore_case_keyword("f").to(Bit::Zero),
        // Ones
        just("1").to(Bit::One),
        ignore_case_keyword("true").to(Bit::One),
        ignore_case_keyword("t").to(Bit::One),
    ))
}

impl TxtPuzzle<BinarioState> for Binario {
    fn read_text<'a>(input: &str) -> read::Result<(Self, BinarioState)> {
        let (cells, entries) =
            cell_entry_grids(bit())
                .parse(input)
                .into_result()
                .map_err(|errs| {
                    read::Error::Parse(errs.into_iter().map(|err| err.to_string()).collect())
                })?;

        let solutions = cells.map_ref(|cell| cell.solution);

        let timer = Timer::default();
        let meta = Metadata::default();

        let puzzle = Binario::new(cells, meta);
        let state = BinarioState::new(solutions, entries, timer);

        Ok((puzzle, state))
    }

    fn write_text(&self, state: &BinarioState) -> String {
        format!("{state}\n{}", self.meta())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use puzzled_io::{
        TxtPuzzle, TxtReader,
        text::read::{cell_entry, grid_row},
    };
    use rstest::rstest;

    use super::*;
    use crate::binario;

    const T: Bit = Bit::One;
    const F: Bit = Bit::Zero;

    #[rstest]
    #[case("0", F)]
    #[case("1", T)]
    #[case("f", F)]
    #[case("F", F)]
    #[case("t", T)]
    #[case("T", T)]
    #[case("false", F)]
    #[case("FAlse", F)]
    fn parse_bit(#[case] input: &str, #[case] expected: Bit) {
        let bit = bit().parse(input).unwrap();
        assert_eq!(bit, expected);
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

    #[rstest]
    fn read(#[files("puzzles/ok/*.txt")] path: PathBuf) {
        let reader = TxtReader::new(false);
        let (_puzzle, _state): (Binario, BinarioState) =
            reader.read_from_path(path).expect("Puzzle is valid");
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
