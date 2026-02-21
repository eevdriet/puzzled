use std::str::Lines;

use crate::io::{Context, Warning, format, read};

#[derive(Debug, Default)]
pub struct PuzState {
    strict: bool,
    pub warnings: Vec<Warning>,
}

impl PuzState {
    pub(crate) fn new(strict: bool) -> Self {
        Self {
            strict,
            warnings: Vec::new(),
        }
    }

    pub(crate) fn ok_or_warn<T>(&mut self, result: read::Result<T>) -> read::Result<Option<T>> {
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

pub(crate) struct TxtState<'a> {
    pub lines: Lines<'a>,

    pub pos: usize,
    pub len: Option<usize>,
}

impl<'a> TxtState<'a> {
    pub(crate) fn next(&mut self) -> Option<&'a str> {
        let line = self.lines.next()?;

        // Add on the previous line length
        if let Some(len) = self.len {
            self.pos += len;
        }

        // Memorize the current line length
        self.len = Some(line.len());

        Some(line)
    }

    pub(crate) fn parse_string<S: Into<String>>(
        &self,
        text: &str,
        context: S,
    ) -> read::Result<String> {
        let text = text.trim();

        if !text.starts_with('"') || !text.ends_with('"') {
            return Err(format::Error::InvalidStringLiteral {
                found: text.to_string(),
            })
            .context(context);
        }

        Ok(text[1..text.len() - 1].to_string())
    }
}
