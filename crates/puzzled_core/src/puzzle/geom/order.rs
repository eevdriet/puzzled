use std::ops;

/// Order which to traverse a [grid](crate::Grid) with
///
/// See the relevant [Wikipedia](https://en.wikipedia.org/wiki/Row-_and_column-major_order) page for how this influences grid traversal
#[derive(Debug, Clone, Copy, Default)]
pub enum Order {
    /// Row-major order, going row by row
    #[default]
    Rows,

    /// Column-major order, going column by column
    Cols,
}

impl Order {
    /// Create an order that is the flipped version of itself
    pub fn flipped(&self) -> Self {
        match self {
            Self::Rows => Self::Cols,
            Self::Cols => Self::Rows,
        }
    }

    /// Flip the order in-place
    pub fn flip(&mut self) {
        *self = self.flipped()
    }
}

impl ops::Neg for Order {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.flipped()
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::Order;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum SerdeOrder {
        Rows,
        Cols,
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Order {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Order::Rows => SerdeOrder::Rows,
                Order::Cols => SerdeOrder::Cols,
            }
            .serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Order {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let data = SerdeOrder::deserialize(deserializer)?;
            let order = match data {
                SerdeOrder::Rows => Order::Rows,
                SerdeOrder::Cols => Order::Cols,
            };

            Ok(order)
        }
    }
}
