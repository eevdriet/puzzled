use chumsky::{Parser, extra::Err, prelude::just, text};
use puzzled_core::Version;

use crate::text::read::ParseError;

pub fn version<'a>() -> impl Parser<'a, &'a str, Version, Err<ParseError<'a>>> + Clone {
    text::int(10)
        .then_ignore(just('.'))
        .then(text::int(10))
        .try_map(|(major_str, minor_str): (&'a str, &'a str), span| {
            let major = major_str
                .parse::<u8>()
                .map_err(|err| ParseError::custom(span, err.to_string()))?;

            let minor = minor_str
                .parse::<u8>()
                .map_err(|err| ParseError::custom(span, err.to_string()))?;

            let version = Version::new(major, minor);
            Ok(version)
        })
}
