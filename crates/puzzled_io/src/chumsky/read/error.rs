use chumsky::{
    DefaultExpected,
    error::{Error as ChumskyError, LabelError},
    span::SimpleSpan,
    util::MaybeRef,
};

use crate::format;

type Span = SimpleSpan<usize>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Chumsky")]
    ExpectedFound {
        span: Span,
        expected: Vec<DefaultExpected<'static, char>>,
        found: Option<char>,
    },

    #[error("{format}")]
    Format { format: format::Error, span: Span },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl Error {
    pub fn new(format: format::Error, span: Span) -> Self {
        Self::Format { format, span }
    }
}

impl<'a> ChumskyError<'a, &'a str> for Error {
    fn merge(mut self, mut other: Self) -> Self {
        if let (
            Self::ExpectedFound { expected, .. },
            Self::ExpectedFound {
                expected: expected_other,
                ..
            },
        ) = (&mut self, &mut other)
        {
            expected.append(expected_other);
        }
        self
    }
}

impl<'a> LabelError<'a, &'a str, DefaultExpected<'a, char>> for Error {
    fn expected_found<Iter: IntoIterator<Item = DefaultExpected<'a, char>>>(
        expected: Iter,
        found: Option<MaybeRef<'a, char>>,
        span: Span,
    ) -> Self {
        Self::ExpectedFound {
            span,
            expected: expected.into_iter().map(|e| e.into_owned()).collect(),
            found: found.as_deref().copied(),
        }
    }
}
