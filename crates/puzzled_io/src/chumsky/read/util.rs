use chumsky::{
    Parser,
    prelude::{just, none_of},
};

pub fn quoted_string<'a>() -> impl Parser<'a, &'a str, &'a str> {
    just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
}
