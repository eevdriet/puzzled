use std::{iter::Inspect, str::Lines};

use thiserror::Error;

use crate::io::{Error, ErrorKind, PuzState, Result, Span, TxtState};

#[derive(Debug, Error, Clone, Copy)]
pub enum ReadError {
    #[error("Unexpected EOF")]
    UnexpectedEof,

    #[error("Unterminated string of len {len}")]
    UnterminatedStr { len: usize },
}

impl<'a> PuzState<'a> {
    pub(crate) fn reached_eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub(crate) fn take<S: AsRef<str>>(&mut self, len: usize, context: S) -> Result<&'a [u8]> {
        let end = self.pos + len;
        let slice = self.input.get(self.pos..end).ok_or_else(|| Error {
            span: self.pos..self.input.len(),
            kind: ReadError::UnexpectedEof.into(),
            context: context.as_ref().into(),
        })?;

        Ok(slice)
    }

    pub(crate) fn read<S: AsRef<str>>(&mut self, len: usize, context: S) -> Result<&'a [u8]> {
        let slice = self.take(len, context)?;
        self.pos += len;

        Ok(slice)
    }

    pub(crate) fn read_u8<S: AsRef<str>>(&mut self, context: S) -> Result<u8> {
        let bytes = self.read(1, context)?;

        Ok(bytes[0])
    }

    pub(crate) fn read_u16<S: AsRef<str>>(&mut self, context: S) -> Result<u16> {
        let bytes = self.read(2, context)?;
        let bytes = [bytes[0], bytes[1]];

        Ok(u16::from_le_bytes(bytes))
    }

    pub(crate) fn read_str<S: AsRef<str> + Clone>(&mut self, context: S) -> Result<&'a [u8]> {
        // Read until the end of the string or EOF
        let start = self.pos;

        while self.pos < self.input.len() && self.input[self.pos] != b'\0' {
            self.pos += 1;
        }

        // Return error for EOF
        if self.pos >= self.input.len() {
            return Err(Error {
                span: start..self.pos,
                kind: ReadError::UnexpectedEof.into(),
                context: context.as_ref().to_string(),
            });
        }

        // Otherwise, collect and parse the string (including trailing \0)
        self.pos += 1;
        let bytes = &self.input[start..self.pos];

        Ok(bytes)
    }

    pub(crate) fn read_fixed_len_str<S: AsRef<str>>(
        &mut self,
        len: usize,
        context: S,
    ) -> Result<&'a [u8]> {
        // Read the requested amount of bytes for the string
        let start = self.pos;
        let bytes = self.read(len, &context)?;

        // Make sure it is terminated properly with a trailing \0
        bytes.last().filter(|&&b| b == b'\0').ok_or_else(|| Error {
            span: start..self.pos,
            kind: ReadError::UnterminatedStr { len }.into(),
            context: context.as_ref().into(),
        })?;

        Ok(bytes)
    }

    pub(crate) fn skip(&mut self, count: usize, context: &'static str) -> Result<()> {
        self.read(count, context)?;

        Ok(())
    }

    pub(crate) fn build_string(mut bytes: &[u8]) -> String {
        if let Some(stripped) = bytes.strip_suffix(&[0]) {
            bytes = stripped;
        }

        match std::str::from_utf8(bytes) {
            // Check if the string can be parsed as UTF-8 directly
            Ok(s) => s.to_string(),

            // Otherwise, apply the Windows-1252 character mapping
            Err(_) => bytes.iter().map(|&b| windows_1252_to_char(b)).collect(),
        }
    }
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

    pub(crate) fn parse_string<S: Into<String>>(&self, text: &str, context: S) -> Result<String> {
        let text = text.trim();

        if !text.starts_with('"') || !text.ends_with('"') {
            return Err(Error {
                span: self.pos..self.pos + text.len(),
                kind: ErrorKind::Custom("[...]".to_string()),
                context: context.into(),
            });
        }

        Ok(text[1..text.len() - 1].to_string())
    }
}

pub(crate) fn windows_1252_to_char(byte: u8) -> char {
    // Windows-1252 character mapping for bytes 128-159 that differ from ISO-8859-1
    // Legacy .puz files often use Windows-1252 encoding for special characters
    match byte {
        // Standard ASCII range (0-127) maps directly
        0..=127 => byte as char,
        // Windows-1252 specific mappings for 128-159 range
        128 => '€',        // Euro sign
        129 => '\u{0081}', // Unused
        130 => '‚',        // Single low-9 quotation mark
        131 => 'ƒ',        // Latin small letter f with hook
        132 => '„',        // Double low-9 quotation mark
        133 => '…',        // Horizontal ellipsis
        134 => '†',        // Dagger
        135 => '‡',        // Double dagger
        136 => 'ˆ',        // Modifier letter circumflex accent
        137 => '‰',        // Per mille sign
        138 => 'Š',        // Latin capital letter S with caron
        139 => '‹',        // Single left-pointing angle quotation mark
        140 => 'Œ',        // Latin capital ligature OE
        141 => '\u{008D}', // Unused
        142 => 'Ž',        // Latin capital letter Z with caron
        143 => '\u{008F}', // Unused
        144 => '\u{0090}', // Unused
        145 => '\u{2018}', // Left single quotation mark
        146 => '\u{2019}', // Right single quotation mark
        147 => '\u{201C}', // Left double quotation mark
        148 => '\u{201D}', // Right double quotation mark
        149 => '•',        // Bullet
        150 => '–',        // En dash
        151 => '—',        // Em dash
        152 => '˜',        // Small tilde
        153 => '™',        // Trade mark sign
        154 => 'š',        // Latin small letter s with caron
        155 => '›',        // Single right-pointing angle quotation mark
        156 => 'œ',        // Latin small ligature oe
        157 => '\u{009D}', // Unused
        158 => 'ž',        // Latin small letter z with caron
        159 => 'Ÿ',        // Latin capital letter Y with diaeresis
        // ISO-8859-1 range (160-255) is identical to Windows-1252
        160..=255 => byte as char,
    }
}
