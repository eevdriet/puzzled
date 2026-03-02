use chumsky::{
    Parser,
    error::EmptyErr,
    extra::Err,
    prelude::{choice, empty, just},
    text,
};
use puzzled_core::{Metadata, Timer, Version};

use crate::chumsky::read::quoted_string;

mod timer;
mod version;

pub fn metadata<'a, T, P>() -> impl Parser<'a, &'a str, Metadata, Err<EmptyErr>> + Clone {
    empty().map(|_| Metadata::default())
}

enum MetaField<'a> {
    String { key: &'a str, val: &'a str },
    Version(Version),
    Timer(Timer),
}

// fn meta_field<'a, T, P>(value: P) -> impl Parser<'a, &'a str, MetaField<'a>, Err<EmptyErr>> + Clone
// where
//     P: Parser<'a, &'a str, T, Err<EmptyErr>> + Clone,
// {
//     let str_prop = choice((
//         just("author"),
//         just("copyright"),
//         just("notes"),
//         just("title"),
//     ));
// }

// fn meta_key_val<'a, T, P>(value: Option<P>) -> impl Parser<'a, &'a str, (&'a str, T), Err<EmptyErr>>
// where
//     P: Parser<'a, &'a str, T, Err<EmptyErr>> + Clone,
// {
//     let meta_key = text::ascii::ident::<&str, Err<EmptyErr>>()
//         .padded()
//         .then_ignore(just(':'));
//
//     match value {
//         None => meta_key.then(quoted_string().padded()),
//         Some(value) => meta_key.then(quoted_string().padded().then(value)),
//     }
// }
