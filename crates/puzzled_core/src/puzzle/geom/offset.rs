use std::ops::Neg;

/// Amounts by which to move a [`Position`](crate::Position).
///
/// Positive numbers move to the right/bottom and negative to the left/top.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Offset {
    /// How many rows to move over by
    pub rows: isize,

    /// How many columns to move over by
    pub cols: isize,
}

impl Offset {
    /// A zero offset
    pub const ZERO: Self = Self { cols: 0, rows: 0 };
    /// A left offset
    pub const LEFT: Self = Self { cols: -1, rows: 0 };
    /// A right offset
    pub const RIGHT: Self = Self { cols: 1, rows: 0 };
    /// An up offset
    pub const UP: Self = Self { cols: 0, rows: -1 };
    /// A down offset
    pub const DOWN: Self = Self { cols: 0, rows: 1 };
}

impl Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        Self {
            rows: -self.rows,
            cols: -self.cols,
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::Offset;

    type OffsetData = [isize; 2];

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Offset {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let data: OffsetData = [self.rows, self.cols];
            data.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Offset {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let [rows, cols] = OffsetData::deserialize(deserializer)?;
            let offset = Offset { rows, cols };

            Ok(offset)
        }
    }
}
