use std::{iter::Peekable, str::Lines};

use crate::io::text;

#[derive(Debug)]
pub(crate) struct TxtState<'a> {
    pub(crate) lines: Peekable<Lines<'a>>,
    pub(crate) pos: usize,
    pub(crate) len: Option<usize>,
    pub(crate) peeked: Option<&'a str>,
}

impl<'a> TxtState<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines().peekable(),
            pos: 0,
            len: None,
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&'a str> {
        if self.peeked.is_none() {
            self.peeked = self.lines.next();
        }
        self.peeked
    }

    pub fn next(&mut self) -> Option<&'a str> {
        let line = self.peeked.take().or_else(|| self.lines.next())?;

        if let Some(len) = self.len {
            self.pos += len;
        }

        self.len = Some(line.len());

        Some(line)
    }

    pub(crate) fn parse_string(&self, text: &str) -> text::Result<String> {
        let text = text.trim();

        if !text.starts_with('"') || !text.ends_with('"') {
            return Err(text::Error::InvalidStringLiteral {
                found: text.to_string(),
            });
        }

        Ok(text[1..text.len() - 1].to_string())
    }
}
