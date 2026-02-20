mod position;

use std::{fmt, ops};

pub use position::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Line {
    Row(usize),
    Col(usize),
}

impl Line {
    pub fn line(&self) -> usize {
        match self {
            Self::Row(row) => *row,
            Self::Col(col) => *col,
        }
    }

    pub fn with_line(&self, line: usize) -> Self {
        match self {
            Self::Row(_) => Self::Row(line),
            Self::Col(_) => Self::Col(line),
        }
    }

    pub fn offset(&self, offset: isize) -> Self {
        let line = self.line() as isize;
        let line = line + offset;

        self.with_line(line as usize)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (num, char) = match self {
            Self::Row(row) => (row, 'R'),
            Self::Col(col) => (col, 'C'),
        };

        write!(f, "{char}{num}")
    }
}

impl ops::Add<isize> for Line {
    type Output = Self;

    /// Moves the position by the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn add(self, offset: isize) -> Self {
        self.offset(offset)
    }
}

impl ops::Sub<isize> for Line {
    type Output = Self;

    /// Moves the position by the inverse of the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn sub(self, offset: isize) -> Self {
        self.offset(-offset)
    }
}

impl ops::AddAssign<isize> for Line {
    /// Moves the position in place by the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn add_assign(&mut self, offset: isize) {
        *self = *self + offset;
    }
}

impl ops::SubAssign<isize> for Line {
    /// Moves the position in place by the given offset.
    ///
    /// Values that would move the position outside the `u8` range are clamped
    fn sub_assign(&mut self, offset: isize) {
        *self = *self - offset;
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::Line;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum LineData {
        Row(usize),
        Col(usize),
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Line {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let data = match self {
                Self::Row(row) => LineData::Row(*row),
                Self::Col(col) => LineData::Col(*col),
            };

            data.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Line {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let data = LineData::deserialize(deserializer)?;
            let line = match data {
                LineData::Row(row) => Line::Row(row),
                LineData::Col(col) => Line::Col(col),
            };

            Ok(line)
        }
    }
}
