mod fills;
mod mask;

pub use fills::*;
pub use mask::*;

use std::fmt::Debug;

use crate::ColorId;

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
    pub const fn from_byte(byte: u8) -> Self {
        match byte {
            0 => Fill::Blank,
            1 => Fill::Cross,
            b => Fill::Color(b as usize),
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

    pub fn byte(&self) -> usize {
        match self {
            Fill::Blank => b'.' as usize,
            Fill::Cross => 0,
            Fill::Color(id) => *id,
        }
    }

    pub fn as_key(&self) -> usize {
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
                id if color_count.is_some() && *id > color_count.unwrap() => None,

                // Use 0-9 for first 10 colors
                id @ 1..=9 => char::from_u32(b'0' as u32 + *id as u32),

                // Use alphabet for next 25 colors (skip 'x' for cross)
                id @ 10..24 | id @ 25..=26 => char::from_u32(b'a' as u32 + *id as u32 - 9),
                24 => Some(','),

                _ => None,
            },
        }
    }

    pub const fn from_char_const(char: char) -> Self {
        let id = match char {
            // Non-colors
            '.' => return Fill::Blank,
            '0' | 'x' | 'X' => return Fill::Cross,
            // 1. Numbers
            col @ '1'..='9' => (col as u8 - b'1') as usize,
            // 2. Lowercase letters
            col @ 'a'..'x' => (col as u8 - b'a' + 9) as usize,
            col @ 'y'..='z' => (col as u8 - b'y' + 9 + 23) as usize,
            // 3. Uppercase letters
            col @ 'A'..'X' => (col as u8 - b'A' + 9 + 25 + 23) as usize,
            col @ 'Y'..='Z' => (col as u8 - b'Y' + 9 + 25 + 25) as usize,

            // Unknown
            _ => {
                panic!("Found unknown character to represent Fill::Color");
            }
        };

        Fill::Color(id)
    }

    pub const fn from_str_const(str: &str) -> Self {
        let bytes = str.as_bytes();

        if bytes.is_empty() {
            return Fill::Blank;
        }

        if bytes.len() != 1 {
            panic!("Fill must be represented by 0 (blank) or 1 characters");
        }

        Self::from_char_const(bytes[0] as char)
    }
}

impl From<Fill> for Option<usize> {
    fn from(fill: Fill) -> Self {
        match fill {
            Fill::Blank => None,
            Fill::Cross => Some(0),
            Fill::Color(col) => Some(col),
        }
    }
}

impl From<&Fill> for Fill {
    fn from(fill: &Fill) -> Self {
        *fill
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::Fill;

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Fill {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Fill::Blank => 0,
                Fill::Cross => 1,
                Fill::Color(color) => *color,
            }
            .serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Fill {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let fill = match usize::deserialize(deserializer)? {
                0 => Fill::Blank,
                1 => Fill::Cross,
                color => Fill::Color(color),
            };

            Ok(fill)
        }
    }
}
