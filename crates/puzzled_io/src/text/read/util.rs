use chumsky::{
    Parser,
    error::Rich,
    extra::Err,
    prelude::{just, none_of},
    text::ident,
};

use crate::text::read::ParseError;

pub fn quoted_string<'a>() -> impl Parser<'a, &'a str, &'a str, Err<ParseError<'a>>> + Clone {
    just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
}

/// A case-insensitive variant of chumsky::text::keyword
pub fn ignore_case_keyword<'a>(
    keyword: &'static str,
) -> impl Parser<'a, &'a str, (), Err<ParseError<'a>>> + Clone {
    ident().try_map(|s: &str, span| {
        s.eq_ignore_ascii_case(keyword)
            .then_some(())
            .ok_or_else(|| Rich::custom(span, "Invalid keyword"))
    })
}
