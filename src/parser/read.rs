use std::borrow::Cow;

use thiserror::Error;

use crate::{Error, Parser, Result};

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("Encountered unexpected EOF while trying to read '{context}' at position {pos}")]
    UnexpectedEof { context: String, pos: usize },

    #[error("Read unterminated string '{context}' of len {len} at position {pos}")]
    UnterminatedStr {
        context: String,
        len: usize,
        pos: usize,
    },
}

impl<'a> Parser<'a> {
    pub(crate) fn take<S: AsRef<str>>(&mut self, len: usize, context: S) -> Result<&'a [u8]> {
        let end = self.pos + len;
        let slice = self.input.get(self.pos..end).ok_or_else(|| {
            Into::<Error>::into(ReadError::UnexpectedEof {
                context: context.as_ref().into(),
                pos: self.pos,
            })
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
            return Err(ReadError::UnexpectedEof {
                pos: self.pos,
                context: context.as_ref().to_string(),
            }
            .into());
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
        let pos = self.pos;
        let bytes = self.read(len, &context)?;

        // Make sure it is terminated properly with a trailing \0
        bytes.last().filter(|&&b| b == b'\0').ok_or_else(|| {
            Into::<Error>::into(ReadError::UnterminatedStr {
                context: context.as_ref().into(),
                len,
                pos,
            })
        })?;

        Ok(bytes)
    }

    pub(crate) fn skip(&mut self, count: usize, context: &'static str) -> Result<()> {
        self.read(count, context)?;

        Ok(())
    }
}

pub(crate) fn parse_string(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        // Check if the string can be parsed as UTF-8 directly
        Ok(s) => s.to_string(),

        // Otherwise, apply the Windows-1252 character mapping
        Err(_) => bytes.iter().map(|&b| windows_1252_to_char(b)).collect(),
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
