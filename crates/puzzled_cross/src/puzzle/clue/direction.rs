use std::{fmt, str::FromStr};

/// Direction which a [clue](crate::Clue) can be placed in a [puzzle](crate::Crossword)
///
/// Together with the *clue number*, the [`Direction`] can [identify](crate::ClueId) where a clue should be placed.
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

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::Direction;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum SerdeDirection {
        Across,
        Down,
    }

    impl Serialize for Direction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Direction::Across => SerdeDirection::Across,
                Direction::Down => SerdeDirection::Down,
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let data = SerdeDirection::deserialize(deserializer)?;
            let direction = match data {
                SerdeDirection::Across => Direction::Across,
                SerdeDirection::Down => Direction::Down,
            };

            Ok(direction)
        }
    }
}
