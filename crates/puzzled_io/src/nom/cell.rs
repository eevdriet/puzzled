use nom::{
    IResult, Parser,
    branch::alt,
    character::{
        char,
        complete::{one_of, space0},
    },
    combinator::opt,
    error::Error,
    multi::many0,
    sequence::{delimited, preceded},
};
use puzzled_core::{Cell, CellStyle, Entry, MISSING_ENTRY_CHAR};

pub fn cell_entry<'a, T, P>(input: &'a str, value: P) -> IResult<&'a str, (Cell<T>, Entry<T>)>
where
    P: Parser<&'a str, Output = T, Error = Error<&'a str>> + Clone,
{
    // Get a solution from anything but - (missing solution character)
    let (input, solution) = alt((
        char(MISSING_ENTRY_CHAR).map(|_| None),
        value.clone().map(Some),
    ))
    .parse(input)?;

    // Get the cell style
    let (input, style) = opt(cell_style).parse(input)?;
    let style = style.unwrap_or_default();

    // Optionally get the entry as well
    let (input, entry) =
        opt(preceded(space0, delimited(char('('), value, char(')')))).parse(input)?;

    let cell = Cell::new_with_style(solution, style);
    let entry = Entry::new_with_style(entry, style);

    Ok((input, (cell, entry)))
}

pub fn cell_style(input: &str) -> IResult<&str, CellStyle> {
    // Get all markers and apply them one by one
    let (input, markers) = many0(one_of("*@~!")).parse(input)?;

    let mut style = CellStyle::default();

    for marker in markers {
        match marker {
            '*' => style |= CellStyle::REVEALED,
            '!' => style |= CellStyle::INCORRECT,
            '~' => style |= CellStyle::PREVIOUSLY_INCORRECT,
            '@' => style |= CellStyle::CIRCLED,
            _ => {}
        }
    }

    Ok((input, style))
}
