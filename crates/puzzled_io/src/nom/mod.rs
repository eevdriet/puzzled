mod cell;
mod grid;
mod metadata;
mod square;

pub use cell::*;
pub use grid::*;
pub use metadata::*;
pub use square::*;

use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until},
    sequence::delimited,
};

pub fn quoted_string(input: &str) -> IResult<&str, &str> {
    let (input, output) = delimited(tag("\""), take_until("\""), tag("\"")).parse(input)?;

    Ok((input, output))
}
