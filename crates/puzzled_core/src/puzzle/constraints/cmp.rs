use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comparison {
    Less,
    LessOrEqual,
    Equal,
    GreaterOrEqual,
    Greater,
    NotEqual,
}

impl Comparison {
    pub fn satisfies<T>(&self, lhs: &T, rhs: &T) -> bool
    where
        T: PartialEq + PartialOrd,
    {
        match self {
            Comparison::Less => lhs.lt(rhs),
            Comparison::LessOrEqual => lhs.le(rhs),
            Comparison::Equal => lhs.eq(rhs),
            Comparison::GreaterOrEqual => lhs.ge(rhs),
            Comparison::Greater => lhs.gt(rhs),
            Comparison::NotEqual => lhs.ne(rhs),
        }
    }
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Less => "<",
                Self::LessOrEqual => "<=",
                Self::Equal => "==",
                Self::NotEqual => "!=",
                Self::GreaterOrEqual => ">=",
                Self::Greater => ">",
            }
        )
    }
}

impl From<Ordering> for Comparison {
    fn from(ord: Ordering) -> Self {
        match ord {
            Ordering::Less => Comparison::Less,
            Ordering::Equal => Comparison::Equal,
            Ordering::Greater => Comparison::Greater,
        }
    }
}
