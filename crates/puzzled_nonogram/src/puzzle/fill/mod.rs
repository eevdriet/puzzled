mod mask;

pub use mask::*;

use std::fmt::Debug;

use crate::ColorId;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Fill error: {0}")]
pub enum FillError {
    #[error("Can only create fill from a single character")]
    InvalidLen,

    #[error("Invalid character '{0}' used to create fill, only 0..=9, a..=z and A..=Z are allowed")]
    InvalidChar(char),

    #[error(
        "Invalid id '{0} used to create fill, only ASCII 0..=9, a..=z, A..=Z and . are allowed"
    )]
    InvalidId(u32),
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fill {
    /// Not yet filled out cell
    #[default]
    Blank,

    // Crossed out cell
    Cross,

    // Colored cell
    Color(ColorId),
}

impl Fill {
    pub const fn decode_char(ch: char) -> Result<Self, FillError> {
        match ch {
            // Non-colors
            '.' => Ok(Fill::Blank),
            '0' | 'x' | 'X' => Ok(Fill::Cross),

            id @ ('1'..='9' | 'a'..='z' | 'A'..='Z') => Ok(Fill::Color(id as u32)),

            // Unknown
            _ => Err(FillError::InvalidChar(ch)),
        }
    }

    pub const fn decode_str(str: &str) -> Result<Self, FillError> {
        let bytes = str.as_bytes();

        if bytes.is_empty() {
            return Ok(Fill::Blank);
        }

        if bytes.len() != 1 {
            return Err(FillError::InvalidLen);
        }

        Self::decode_char(bytes[0] as char)
    }

    pub fn index(&self) -> Result<usize, FillError> {
        match *self {
            Fill::Blank => Ok(0),
            Fill::Cross => Ok(1),
            Fill::Color(id) => {
                let color_char = char::from_u32(id).ok_or(FillError::InvalidId(id))?;

                let id = match color_char {
                    '.' => 0,             // Blank
                    '0' | 'x' | 'X' => 1, // Cross

                    // Colors
                    // 1. Numbers
                    col @ '1'..='9' => (col as u8 - b'1') as usize,
                    // 2. Lowercase letters
                    col @ 'a'..'x' => (col as u8 - b'a' + 9) as usize,
                    col @ 'y'..='z' => (col as u8 - b'y' + 9 + 23) as usize,
                    // 3. Uppercase letters
                    col @ 'A'..'X' => (col as u8 - b'A' + 9 + 25 + 23) as usize,
                    col @ 'Y'..='Z' => (col as u8 - b'Y' + 9 + 25 + 25) as usize,
                    _ => return Err(FillError::InvalidChar(color_char)),
                };

                Ok(id)
            }
        }
    }

    pub fn is_color(&self) -> bool {
        matches!(self, Fill::Color(_))
    }

    pub fn symbol(&self) -> char {
        match self {
            Fill::Blank => '◦',
            Fill::Cross => '×',
            // Fill::Color(_) => '█',
            Fill::Color(_) => '■',
        }
    }

    pub fn as_key(&self) -> u32 {
        match self {
            Fill::Blank => 0,
            Fill::Cross => 1,
            Fill::Color(id) => id + 2,
        }
    }

    pub fn key(&self, color_count: Option<usize>) -> Option<char> {
        match self {
            // Default characters for blanks and crosses
            Fill::Blank => Some('.'),
            Fill::Cross => Some('x'),

            // 0-9 for <=10 colors (most puzzles)
            Fill::Color(id) => match id {
                // Color is undefined
                id if color_count.is_some() && *id > color_count.unwrap() as u32 => None,

                // Use 0-9 for first 10 colors
                id @ 1..=9 => char::from_u32(b'0' as u32 + *id),

                // Use alphabet for next 25 colors (skip 'x' for cross)
                id @ 10..24 | id @ 25..=26 => char::from_u32(b'a' as u32 + *id - 9),
                24 => Some(','),

                _ => None,
            },
        }
    }
}

impl TryFrom<char> for Fill {
    type Error = FillError;

    fn try_from(fill_char: char) -> Result<Self, Self::Error> {
        Self::decode_char(fill_char)
    }
}

impl TryFrom<Fill> for char {
    type Error = FillError;

    fn try_from(fill: Fill) -> Result<Self, Self::Error> {
        match fill {
            Fill::Blank => Ok('.'),
            Fill::Cross => Ok('X'),
            Fill::Color(id) => {
                let color_char = char::from_u32(id).ok_or(FillError::InvalidId(id))?;

                match color_char {
                    '.' | '0'..='9' | 'a'..='z' | 'A'..='Z' => Ok(color_char),
                    _ => Err(FillError::InvalidId(id)),
                }
            }
        }
    }
}

impl TryFrom<&str> for Fill {
    type Error = FillError;

    fn try_from(fill_str: &str) -> Result<Self, Self::Error> {
        Self::decode_str(fill_str)
    }
}

impl From<&Fill> for Fill {
    fn from(fill: &Fill) -> Self {
        *fill
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize, de, ser};

    use crate::Fill;

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Fill {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let fill_char = char::try_from(*self).map_err(ser::Error::custom)?;
            fill_char.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Fill {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let fill_char = char::deserialize(deserializer)?;
            let fill = Fill::decode_char(fill_char).map_err(de::Error::custom)?;

            Ok(fill)
        }
    }
}
