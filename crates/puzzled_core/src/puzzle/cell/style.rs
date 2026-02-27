use std::{fmt, str::FromStr};

use bitflags::bitflags;

bitflags! {
    /// Style that changes the way a [cell](Cell) is displayed
    ///
    /// The style is represented as *bit flags* such that multiple styles can simultaneously be set.
    /// Currently, the 4 styles that are defined are
    /// - [`PREVIOUSLY_INCORRECT`](CellStyle::PREVIOUSLY_INCORRECT) (`0x10`) for cells that previously contained an [incorrect](Cell::is_correct) guess
    /// - [`INCORRECT`](CellStyle::INCORRECT) (`0x20`) for cells that currently contain an [incorrect](Cell::is_correct) guess
    /// - [`REVEALED`](CellStyle::REVEALED) (`0x40`) for cells that are manually [revealed](Cell::reveal) by the user to show their solution
    /// - [`CIRCLED`](CellStyle::CIRCLED) (`0x80`) for cells that are circled
    ///
    /// The definitions derive from the **GEXT data section** of the [*.puz spefication](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki).
    ///
    /// ```rust
    /// use puzzled::crossword::{Cell, CellStyle, Solution, Reveal};
    ///
    /// let style = CellStyle::INCORRECT | CellStyle::CIRCLED;
    /// let mut cell = Cell::new_styled(Solution::Letter('A'), style);
    /// assert!(!cell.is_correct());
    /// assert!(cell.is_circled());
    ///
    /// assert!(!cell.is_revealed());
    /// cell.reveal();
    /// assert!(cell.is_revealed());
    /// ```
    #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
    #[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
    pub struct CellStyle: u8 {
        /// [Cell] is initially revealed
        ///
        /// This is sometimes required to ensure a puzzle is solvable from its initial state
        const INITIALLY_REVEALED   = 1 << 3; //

        /// [Cell] previously contained an [incorrect](Cell::is_correct) guess
        const PREVIOUSLY_INCORRECT = 1 << 4; // ~

        /// [Cell] currently contains an [incorrect](Cell::is_correct) guess
        const INCORRECT            = 1 << 5; // !

        /// [Cell] is manually [revealed](Cell::reveal) by the user to show its solution
        const REVEALED             = 1 << 6; // *

        /// [Cell] is circled.
        ///
        /// This is commonly used for solutions that reveal an inner solution.
        /// E.g. `SHOC(K)(I)(N)(G)(P)(I)(N)K` reveals `KINGPIN`
        const CIRCLED              = 1 << 7; // @
    }
}

impl CellStyle {
    pub fn initial(&self) -> Self {
        let mut style = *self;

        style -= CellStyle::REVEALED;
        style -= CellStyle::PREVIOUSLY_INCORRECT;
        style -= CellStyle::INCORRECT;

        style
    }
}

impl fmt::Display for CellStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let styles = [
            (CellStyle::CIRCLED, '@'),
            (CellStyle::REVEALED, '*'),
            (CellStyle::INCORRECT, '!'),
            (CellStyle::PREVIOUSLY_INCORRECT, '~'),
        ];

        for (style, ch) in styles {
            if self.contains(style) {
                write!(f, "{ch}")?;
            }
        }

        Ok(())
    }
}

impl FromStr for CellStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut style = CellStyle::default();

        for char in s.chars() {
            match char {
                '@' => style |= CellStyle::CIRCLED,
                '*' => style |= CellStyle::REVEALED,
                '!' => style |= CellStyle::INCORRECT,
                '~' => style |= CellStyle::PREVIOUSLY_INCORRECT,
                ch if ch.is_whitespace() => {}
                _ => return Err(()),
            }
        }

        Ok(style)
    }
}

// #[cfg(feature = "serde")]
// mod serde_impl {
//     use serde::{Deserialize, Serialize};
//
//     use crate::CellStyle;
//
//     #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
//     impl Serialize for CellStyle {
//         fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer,
//         {
//             self.0.serialize(serializer)
//         }
//     }
//
//     #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
//     impl<'de> Deserialize<'de> for CellStyle {
//         fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where
//             D: serde::Deserializer<'de>,
//         {
//             let inner = u8::deserialize(deserializer)?;
//             Ok(Self(inner))
//         }
//     }
// }
