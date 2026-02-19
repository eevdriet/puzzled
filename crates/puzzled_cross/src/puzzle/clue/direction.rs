use std::{fmt, str::FromStr};

/// Direction which a [clue](crate::Clue) can be placed in a [puzzle](crate::Crossword)
///
/// Together with the *clue number*, the [`Direction`] can [identify](crate::ClueId) where a clue should be placed.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    /// Across direction (horizontal)
    #[default]
    Across,

    /// Down direction (vertical)
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Across => 'A',
                Direction::Down => 'D',
            }
        )
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Direction::Across),
            "D" => Ok(Direction::Across),
            _ => Err(format!("Expected \"A\" or \"D\", found {s}")),
        }
    }
}
