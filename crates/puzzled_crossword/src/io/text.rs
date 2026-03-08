use std::str::FromStr;

use chumsky::{
    IterParser, Parser,
    extra::Err,
    prelude::{end, group, just, one_of},
    text,
};
use puzzled_core::{Metadata, Timer};
use puzzled_io::{
    TxtPuzzle,
    text::read::{self, ParseError, quoted_string, square_entry_grids},
};

use crate::{ClueDirection, ClueSpec, Crossword, CrosswordState, Solution};

pub fn solution<'a>() -> impl Parser<'a, &'a str, Solution, Err<ParseError<'a>>> + Clone {
    text::ident().map(Solution::from)
}

pub fn clue<'a>() -> impl Parser<'a, &'a str, ClueSpec, Err<ParseError<'a>>> + Clone {
    one_of("AD")
        .padded()
        .then_ignore(just(":").padded())
        .then(quoted_string())
        .try_map(|(dir, clue), span| {
            let dir_str = dir.to_string();
            let dir = ClueDirection::from_str(dir_str.as_str())
                .map_err(|err| ParseError::custom(span, err.to_string()))?;

            Ok(ClueSpec::new(dir, clue))
        })
}

pub fn clues<'a>() -> impl Parser<'a, &'a str, Vec<ClueSpec>, Err<ParseError<'a>>> + Clone {
    just("-")
        .padded()
        .ignore_then(clue())
        .padded() // allow spaces/newlines after each clue
        .repeated()
        .collect()
        .then_ignore(end())
}

impl TxtPuzzle<CrosswordState> for Crossword {
    fn read_text<'a>(input: &str) -> read::Result<(Self, CrosswordState)> {
        let ((squares, entries), clues) = group((square_entry_grids(solution()), clues()))
            .parse(input)
            .into_result()
            .map_err(|errs| {
                read::Error::Parse(errs.into_iter().map(|err| err.to_string()).collect())
            })?;

        let solutions =
            squares.map_ref(|square| square.map_ref(|cell| Some(cell.solution.clone())));

        let timer = Timer::default();
        let meta = Metadata::default();

        let mut puzzle = Crossword::from_squares(squares, meta);
        puzzle.insert_clues(clues);

        let state = CrosswordState::new(solutions, entries, timer);

        Ok((puzzle, state))
    }

    fn write_text(&self, state: &CrosswordState) -> String {
        format!("{state}\n{}", self.meta())
    }
}
