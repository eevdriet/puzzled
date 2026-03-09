use std::{fmt, str::FromStr};

use puzzled_core::Direction;

/// ClueDirection which a [clue](crate::Clue) can be placed in a [puzzle](crate::Crossword)
///
/// Together with the *clue number*, the [`ClueDirection`] can [identify](crate::ClueId) where a clue should be placed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClueDirection {
    /// Across direction (horizontal)
    #[default]
    Across,

    /// Down direction (vertical)
    Down,
}

impl fmt::Display for ClueDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ClueDirection::Across => 'A',
                ClueDirection::Down => 'D',
            }
        )
    }
}

impl FromStr for ClueDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(ClueDirection::Across),
            "D" => Ok(ClueDirection::Down),
            _ => Err(format!("Expected \"A\" or \"D\", found {s}")),
        }
    }
}

impl From<Direction> for ClueDirection {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Left | Direction::Right => ClueDirection::Across,
            Direction::Up | Direction::Down => ClueDirection::Down,
        }
    }
}

impl From<ClueDirection> for Direction {
    fn from(clue_dir: ClueDirection) -> Self {
        match clue_dir {
            ClueDirection::Across => Direction::Right,
            ClueDirection::Down => Direction::Down,
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::ClueDirection;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum SerdeClueDirection {
        Across,
        Down,
    }

    impl Serialize for ClueDirection {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                ClueDirection::Across => SerdeClueDirection::Across,
                ClueDirection::Down => SerdeClueDirection::Down,
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for ClueDirection {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let data = SerdeClueDirection::deserialize(deserializer)?;
            let direction = match data {
                SerdeClueDirection::Across => ClueDirection::Across,
                SerdeClueDirection::Down => ClueDirection::Down,
            };

            Ok(direction)
        }
    }
}
