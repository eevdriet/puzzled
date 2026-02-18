use std::fmt;

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
