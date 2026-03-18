use std::{cmp::Ordering as StdOrdering, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ordering {
    Less,
    LessOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
}

impl fmt::Display for Ordering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Less => "<",
                Self::LessOrEqual => "<=",
                Self::Equal => "==",
                Self::GreaterOrEqual => ">=",
                Self::Greater => ">",
            }
        )
    }
}

impl From<StdOrdering> for Ordering {
    fn from(ord: StdOrdering) -> Self {
        match ord {
            StdOrdering::Less => Ordering::Less,
            StdOrdering::Equal => Ordering::Equal,
            StdOrdering::Greater => Ordering::Greater,
        }
    }
}
