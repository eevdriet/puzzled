mod mask;

pub use mask::*;

use serde::Deserialize;
use std::fmt::Debug;

use crate::ColorId;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Fill {
    /// Not yet filled out cell
    #[default]
    #[serde(rename = "blank_char")]
    Blank,

    // Crossed out cell
    #[serde(rename = "cross_char")]
    Cross,

    // Colored cell
    Color(ColorId),
}

impl Fill {
    pub fn symbol(&self) -> char {
        match self {
            Fill::Blank => '◦',
            Fill::Cross => '×',
            // Fill::Color(_) => '█',
            Fill::Color(_) => '■',
        }
    }

    pub fn key(&self, color_count: Option<u16>) -> Option<char> {
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
}

impl From<Fill> for Option<u16> {
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
