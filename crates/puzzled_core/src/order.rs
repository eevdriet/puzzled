use std::ops;

/// Order which to traverse a [grid](crate::Grid) with
///
/// See the relevant [Wikipedia](https://en.wikipedia.org/wiki/Row-_and_column-major_order) page for how this influences grid traversal
#[derive(Debug, Clone, Copy, Default)]
pub enum Order {
    /// Row-major order, going row by row
    #[default]
    RowMajor,

    /// Column-major order, going column by column
    ColMajor,
}

impl Order {
    /// Create an order that is the flipped version of itself
    pub fn flipped(&self) -> Self {
        match self {
            Self::RowMajor => Self::ColMajor,
            Self::ColMajor => Self::RowMajor,
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
