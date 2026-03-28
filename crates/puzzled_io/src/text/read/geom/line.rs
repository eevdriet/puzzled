use chumsky::{
    Parser,
    extra::Err,
    prelude::{choice, just},
    text,
};
use puzzled_core::{Line, Order};

use crate::text::read::ParseError;

pub fn line<'a>() -> impl Parser<'a, &'a str, Line, Err<ParseError<'a>>> + Clone {
    choice((just('R').to(Order::Rows), just('C').to(Order::Cols)))
        .padded()
        .then(text::int(10).from_str().unwrapped())
        .map(|(order, line)| match order {
            Order::Rows => Line::Row(line),
            Order::Cols => Line::Col(line),
        })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("R1", Line::Row(1))]
    fn parse_line_run(#[case] input: &str, #[case] expected: Line) {
        let line = line().parse(input).unwrap();

        assert_eq!(expected, line);
    }
}
