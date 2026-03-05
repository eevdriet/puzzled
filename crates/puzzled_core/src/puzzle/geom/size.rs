use std::ops::{self};

/// Amounts by which to move a [`Position`](crate::Position).
///
/// Positive numbers move to the right/bottom and negative to the left/top.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Size {
    /// How many rows to move over by
    pub rows: usize,

    /// How many columns to move over by
    pub cols: usize,
}

impl ops::Add for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            rows: self.rows + rhs.rows,
            cols: self.cols + rhs.cols,
        }
    }
}

impl ops::Mul<usize> for Size {
    type Output = Size;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            rows: self.rows * rhs,
            cols: self.cols * rhs,
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::Size;

    type SizeData = [usize; 2];

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Size {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let data: SizeData = [self.rows, self.cols];
            data.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Size {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let [rows, cols] = SizeData::deserialize(deserializer)?;
            let offset = Size { rows, cols };

            Ok(offset)
        }
    }
}
