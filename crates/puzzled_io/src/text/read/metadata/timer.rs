use chumsky::{Parser, error::EmptyErr, extra::Err, prelude::empty};
use puzzled_core::Timer;

pub fn timer<'a, T, P>() -> impl Parser<'a, &'a str, Timer, Err<EmptyErr>> + Clone
where
    P: Parser<'a, &'a str, T, Err<EmptyErr>> + Clone,
{
    empty().map(|_| Timer::default())
}
