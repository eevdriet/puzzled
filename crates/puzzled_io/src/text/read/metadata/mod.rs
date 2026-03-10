mod timer;
mod version;

pub use timer::*;
pub use version::*;

use chumsky::{
    IterParser, Parser,
    extra::Err,
    prelude::{choice, just},
};
use puzzled_core::{Metadata, Timer, Version};

use crate::text::read::{ParseError, quoted_string};

pub fn metadata_with_timer<'a>()
-> impl Parser<'a, &'a str, (Metadata, Option<Timer>), Err<ParseError<'a>>> + Clone {
    meta_field()
        .padded()
        .repeated()
        .collect::<Vec<_>>()
        .map(|fields| {
            let mut meta = Metadata::default();
            let mut timer: Option<Timer> = None;

            for field in fields {
                match field {
                    MetaField::Version(version) => {
                        meta = meta.with_version(version);
                    }
                    MetaField::Timer(timer_val) => {
                        timer = Some(timer_val);
                    }
                    MetaField::String { key, val } => match key {
                        "author" => {
                            meta = meta.with_author(val.to_string());
                        }
                        "copyright" => {
                            meta = meta.with_copyright(val.to_string());
                        }
                        "notes" => {
                            meta = meta.with_notes(val.to_string());
                        }
                        "title" => {
                            meta = meta.with_title(val.to_string());
                        }
                        key => unreachable!(
                            "Should have filtered out invalid meta field '{key}' during parsing"
                        ),
                    },
                }
            }

            (meta, timer)
        })
}

pub enum MetaField<'a> {
    String { key: &'a str, val: &'a str },
    Version(Version),
    Timer(Timer),
}

pub fn meta_field<'a>() -> impl Parser<'a, &'a str, MetaField<'a>, Err<ParseError<'a>>> + Clone {
    choice((
        // String properties
        choice((
            meta_str("author"),
            meta_str("copyright"),
            meta_str("notes"),
            meta_str("title"),
        ))
        .map(|(key, val)| MetaField::String { key, val }),
        // Version
        meta_key_val("version", version()).map(MetaField::Version),
        // Timer
        meta_key_val("timer", timer()).map(MetaField::Timer),
    ))
}

fn meta_str<'a>(
    key: &'a str,
) -> impl Parser<'a, &'a str, (&'a str, &'a str), Err<ParseError<'a>>> + Clone {
    just(key)
        .padded()
        .ignore_then(just(':').padded())
        .ignore_then(quoted_string())
        .map(move |val| (key, val))
}

fn meta_key_val<'a, T, P>(
    key: &'a str,
    value: P,
) -> impl Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone
where
    P: Parser<'a, &'a str, T, Err<ParseError<'a>>> + Clone,
{
    just(key)
        .padded()
        .ignore_then(just(':').padded())
        .ignore_then(value)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("author: \"The New York Times\" title: \"2026-03-07-nyt\"")]
    fn test_metadata(#[case] input: &str) {
        if let Err(errs) = metadata_with_timer().parse(input).into_result() {
            panic!("Errors: {errs:?}");
        }
    }
}
