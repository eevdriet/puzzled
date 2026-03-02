use std::fmt;

use chumsky::{Parser, error::EmptyErr, extra::Err, prelude::just};
use puzzled_core::{Cell, Entry, NON_PLAYABLE_CHAR, Square};

use crate::chumsky::read::cell_entry;

pub fn square_entry<'a, T, P>(
    value: P,
) -> impl Parser<'a, &'a str, (Square<Cell<T>>, Square<Entry<T>>), Err<EmptyErr>> + Clone
where
    T: fmt::Debug,
    P: Parser<'a, &'a str, T, Err<EmptyErr>> + Clone,
{
    just(NON_PLAYABLE_CHAR)
        .map(|_| (Square::new_empty(), Square::new_empty()))
        .or(cell_entry(value).map(|(cell, entry)| (Square::new(cell), Square::new(entry))))
}
