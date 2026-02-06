use std::borrow::Cow;

use thiserror::Error;

use crate::{Error, Parser, Result};

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Encountered unexpected EOF while trying to parse '{context}' at position {pos}")]
    UnexpectedEof { context: String, pos: usize },
}

#[derive(Debug)]
pub(crate) struct Region<'a> {
    pub start: usize,
    pub end: usize,
    pub bytes: &'a [u8],
}

impl<'a> Parser<'a> {
    pub(crate) fn read<S: AsRef<str>>(&mut self, len: usize, context: S) -> Result<&'a [u8]> {
        let end = self.pos + len;
        let slice = self.input.get(self.pos..end).ok_or_else(|| {
            Into::<Error>::into(IoError::UnexpectedEof {
                context: context.as_ref().into(),
                pos: self.pos,
            })
        })?;

        self.pos = end;
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

    pub(crate) fn read_str<S: AsRef<str> + Clone>(&mut self, context: S) -> Result<Cow<'a, str>> {
        let start = self.pos;

        while self.pos < self.input.len() && self.input[self.pos] != 0 {
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return Err(IoError::UnexpectedEof {
                pos: self.pos,
                context: context.as_ref().to_string(),
            }
            .into());
        }

        let bytes = &self.input[start..self.pos];

        // Skip trailing \0
        self.pos += 1;

        Ok(parse_str(bytes))
    }

    pub(crate) fn read_fixed_len_str<S: AsRef<str>>(
        &mut self,
        len: usize,
        context: S,
    ) -> Result<Cow<'a, str>> {
        // Read the requested amount of bytes for the string and optionally remove trailing \0
        let bytes = self.read(len, context)?;
        let bytes = bytes.strip_suffix(&[0]).unwrap_or(bytes);

        Ok(parse_str(bytes))
    }

    pub(crate) fn read_to_end(&mut self) -> Result<()> {
        Ok(())
    }

    pub(crate) fn skip(&mut self, count: usize, context: &'static str) -> Result<()> {
        self.read(count, context)?;

        Ok(())
    }

    pub(crate) fn read_region<T>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<T>,
    ) -> Result<(T, Region<'a>)> {
        // Keep track of where the region starts and read the region
        let start = self.pos;
        let value = f(self)?;

        // Then get the end and region from the current position
        let end = self.pos;
        let bytes = &self.input[start..end];

        Ok((value, Region { start, end, bytes }))
    }
}

pub(crate) fn parse_str(bytes: &[u8]) -> Cow<'_, str> {
    match std::str::from_utf8(bytes) {
        // Check if the string can be parsed as UTF-8 directly
        Ok(s) => Cow::Borrowed(s),

        // Otherwise, apply the Windows-1252 character mapping
        Err(_) => Cow::Owned(bytes.iter().map(|&b| windows_1252_to_char(b)).collect()),
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
