use std::time::Duration;

use chumsky::{
    Parser,
    extra::Err,
    prelude::{just, one_of},
    text,
};
use puzzled_core::{Timer, TimerState};

use crate::text::read::ParseError;

pub fn timer<'a>() -> impl Parser<'a, &'a str, Timer, Err<ParseError<'a>>> + Clone {
    text::int(10)
        .then_ignore(just(','))
        .then(one_of("01"))
        .try_map(|(second_str, state_char): (&'a str, char), span| {
            let seconds = second_str
                .parse::<u64>()
                .map_err(|err| ParseError::custom(span, err.to_string()))?;

            let elapsed = Duration::from_secs(seconds);

            let state = TimerState::try_from(state_char)
                .map_err(|err| ParseError::custom(span, err.reason))?;

            let timer = Timer::new(elapsed, state);
            Ok(timer)
        })
}
