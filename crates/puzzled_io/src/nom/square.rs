use nom::{IResult, Parser, branch::alt, character::char, error::Error};
use puzzled_core::{Cell, Entry, NON_PLAYABLE_CHAR, Square};

use crate::nom::cell_entry;

fn square_entry<'a, T, P>(
    input: &'a str,
    value: P,
) -> IResult<&'a str, (Square<Cell<T>>, Square<Entry<T>>)>
where
    P: Parser<&'a str, Output = T, Error = Error<&'a str>> + Clone,
{
    alt((
        char(NON_PLAYABLE_CHAR).map(|_| (Square::new_empty(), Square::new_empty())),
        move |input| {
            cell_entry(input, value.clone())
                .map(|(rest, (cell, entry))| (rest, (Square::new(cell), Square::new(entry))))
        },
    ))
    .parse(input)
}
