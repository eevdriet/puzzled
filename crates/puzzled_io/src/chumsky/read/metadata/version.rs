use chumsky::{Parser, error::EmptyErr, extra::Err, prelude::empty};
use puzzled_core::Version;

pub fn version<'a, T, P>() -> impl Parser<'a, &'a str, Version, Err<EmptyErr>> + Clone
where
    P: Parser<'a, &'a str, T, Err<EmptyErr>> + Clone,
{
    empty().map(|_| Version::default())
}
