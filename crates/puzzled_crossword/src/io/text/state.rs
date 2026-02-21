use std::str::Lines;

use crate::io::text;

#[derive(Debug)]
pub(crate) struct TxtState<'a> {
    pub(crate) lines: Lines<'a>,
    pub(crate) pos: usize,
    pub(crate) len: Option<usize>,
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
