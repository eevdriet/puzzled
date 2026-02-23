use std::{iter::Peekable, str::Lines};

use crate::{format, text::read};

#[derive(Debug)]
pub struct TxtState<'a> {
    pub(crate) strict: bool,
    pub(crate) warnings: Vec<read::Error>,

    pub(crate) lines: Peekable<Lines<'a>>,
    pub(crate) pos: usize,
    pub(crate) len: Option<usize>,
    pub(crate) peeked: Option<&'a str>,
}

impl<'a> TxtState<'a> {
    pub fn new(input: &'a str, strict: bool) -> Self {
        Self {
            strict,
            warnings: Vec::new(),
            lines: input.lines().peekable(),
            pos: 0,
            len: None,
            peeked: None,
        }
    }

    pub fn peek_line(&mut self) -> Option<&'a str> {
        if self.peeked.is_none() {
            self.peeked = self.read_next_nonempty();
        }
        self.peeked
    }

    pub fn next_line(&mut self) -> Option<&'a str> {
        let line = self.peeked.take().or_else(|| self.read_next_nonempty())?;

        // update position tracking if needed
        if let Some(len) = self.len {
            self.pos += len;
        }

        self.len = Some(line.len());

        Some(line)
    }

    pub fn next_prefixed(&mut self, prefix: &str) -> Option<&'a str> {
        let line = self.peek_line()?;
        if !line.starts_with(prefix) {
            return None;
        }

        let line = self.next_line().expect("Already peeked line");
        line.strip_prefix(prefix)
    }

    fn read_next_nonempty(&mut self) -> Option<&'a str> {
        for raw in self.lines.by_ref() {
            let trimmed = raw.trim();

            if !trimmed.is_empty() {
                return Some(trimmed);
            }
        }
        None
    }

    pub fn read_string(&self, text: &str) -> read::Result<String> {
        let text = text.trim();

        if !text.starts_with('"') || !text.ends_with('"') {
            let err = format::StringError::InvalidLiteral {
                found: text.to_string(),
            };
            return Err(format::Error::String(err).into());
        }

        Ok(text[1..text.len() - 1].to_string())
    }

    pub fn ok_or_warn<T>(&mut self, result: read::Result<T>) -> read::Result<Option<T>> {
        match result {
            // Pass through ok/err with strict mode normally
            Ok(val) => Ok(Some(val)),
            Err(err) if self.strict => Err(err),

            // Warn against errors in non-strict mode
            Err(warning) => {
                self.warnings.push(warning);

                Ok(None)
            }
        }
    }
}
