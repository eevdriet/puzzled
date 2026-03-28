use chumsky::{Parser, error::Rich, extra::Err, prelude::any};
use puzzled_core::Color;

use crate::text::read::ParseError;

pub fn color<'a>() -> impl Parser<'a, &'a str, Color, Err<ParseError<'a>>> + Clone {
    any().repeated().to_slice().try_map(|hex: &str, span| {
        Color::hex(hex.trim()).map_err(|err| Rich::custom(span, err.to_string()))
    })
}
