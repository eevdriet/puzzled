use std::fmt;

/// Solution to a [square](Square) that can be used to verify its correctness
///
/// In almost all cases, solutions consist of a single [letter](Self::Letter).
/// However, users may define a [rebus](Self::Rebus) to construct a multi-letter solution.
/// In `*.puz` files, rebuses are defined from the [GRBS and RTBL sections](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Solution {
    /// One-letter solution
    Letter(char),

    /// Multiple-letter solution, a.k.a. a rebus
    Rebus(String),
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Letter(letter) => write!(f, "{letter}"),
            Self::Rebus(rebus) => write!(f, "{rebus}"),
        }
    }
}
