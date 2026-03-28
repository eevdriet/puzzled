use std::collections::HashMap;

use chumsky::{
    IterParser, Parser,
    error::Rich,
    extra::Err,
    prelude::{any, group, just},
    text,
};
use puzzled_core::Line;
use puzzled_io::{
    TxtPuzzle,
    text::read::{self, ParseError, cell, color, grid, line, metadata_with_timer},
};

use crate::{Colors, Fill, Nonogram, Run};

#[derive(Debug, thiserror::Error)]
enum _Error {
    #[error("Fill colors should be specified as `<fill_id> : \"<text>\"`")]
    InvalidColorSpec,
}

pub fn fill<'a>() -> impl Parser<'a, &'a str, Fill, Err<ParseError<'a>>> + Clone {
    any().try_map(|ch, span| {
        Fill::decode_char(ch).map_err(|err| Rich::custom(span, err.to_string()))
    })
}

pub fn run<'a>() -> impl Parser<'a, &'a str, Run, Err<ParseError<'a>>> + Clone {
    text::int(10)
        .from_str()
        .unwrapped()
        .padded()
        .then(fill())
        .map(|(count, fill)| Run::new(fill, count))
}
pub fn line_run<'a>() -> impl Parser<'a, &'a str, (Line, Vec<Run>), Err<ParseError<'a>>> + Clone {
    line().then_ignore(just(':').padded()).then(
        run()
            .padded()
            .separated_by(just(','))
            .at_least(1)
            .collect::<Vec<_>>(),
    )
}

pub fn line_runs<'a>()
-> impl Parser<'a, &'a str, HashMap<Line, Vec<Run>>, Err<ParseError<'a>>> + Clone {
    line_run().padded().repeated().collect()
}

pub fn colors<'a>() -> impl Parser<'a, &'a str, Colors, Err<ParseError<'a>>> + Clone {
    fill()
        .then_ignore(just(':').padded())
        .then(color())
        .repeated()
        .collect()
        .map(Colors::new)
}

impl TxtPuzzle for Nonogram {
    fn read_text(input: &str) -> read::Result<Self> {
        let (cells, line_runs, colors, (meta, _)) = group((
            grid(cell(fill())),
            line_runs(),
            colors(),
            metadata_with_timer(),
        ))
        .parse(input)
        .into_result()
        .map_err(|errs| {
            read::Error::Parse(errs.into_iter().map(|err| err.to_string()).collect())
        })?;

        Ok(Nonogram::new(cells, colors, meta).with_line_runs(line_runs))
    }

    fn write_text(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::Fill;
    use puzzled_core::Line;

    const A: Fill = Fill::Color(b'a' as u32);
    const B: Fill = Fill::Color(b'b' as u32);

    fn a(count: usize) -> Run {
        Run::new(A, count)
    }
    fn b(count: usize) -> Run {
        Run::new(B, count)
    }

    #[rstest]
    #[case("R1: 1 a", Line::Row(1), vec![a(1)])]
    #[case("C1: 1 b", Line::Col(1), vec![b(1)])]
    #[case("R2: 1 b, 4 a, 4 b", Line::Row(2), vec![b(1), a(4), b(4)])]
    fn parse_line_run(#[case] input: &str, #[case] line: Line, #[case] runs: Vec<Run>) {
        let result = line_run().parse(input).unwrap();

        assert_eq!(line, result.0);
        assert_eq!(runs, result.1);
    }
}
