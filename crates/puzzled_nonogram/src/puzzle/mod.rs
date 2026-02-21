mod error;
mod fill;
mod find;
mod rule;
mod run;
mod style;

use bitvec::vec::BitVec;
use derive_more::{Index, IndexMut};
use puzzled_core::Line;
use std::collections::HashMap;

pub use error::*;
pub use fill::*;
pub use find::*;
pub use rule::*;
pub use run::*;
pub use style::*;

#[derive(Debug, Default, Index, IndexMut)]
pub struct Nonogram {
    #[index]
    #[index_mut]
    fills: Fills,

    rules: Rules,
    colors: Vec<Color>,
}

impl Nonogram {
    pub fn new(fills: Fills, rules: Rules, colors: Vec<Color>) -> Self {
        Self {
            fills,
            rules,
            colors,
        }
    }

    pub fn fills(&self) -> &Fills {
        &self.fills
    }

    pub fn fills_mut(&mut self) -> &mut Fills {
        &mut self.fills
    }

    pub fn rules(&self) -> &Rules {
        &self.rules
    }

    pub fn rules_mut(&mut self) -> &mut Rules {
        &mut self.rules
    }

    pub fn colors(&self) -> &Vec<Color> {
        &self.colors
    }

    /// Number of columns in the grid
    pub fn cols(&self) -> usize {
        self.fills.cols()
    }

    /// Number of rows in the grid
    pub fn rows(&self) -> usize {
        self.fills.rows()
    }
}

pub type LineMap<T> = HashMap<Line, T>;

pub type LineMask = BitVec;

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::{Color, Fills, Nonogram, Rules, SerdeRules};

    #[derive(Serialize, Deserialize)]
    struct SerdeNonogram {
        rows: usize,
        cols: usize,

        fills: Fills,
        rules: SerdeRules,
        colors: Vec<Color>,
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Nonogram {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let rules = self.rules.to_serde();
            let fills = self.fills.clone();
            let colors = self.colors.clone();

            SerdeNonogram {
                rules,
                fills,
                colors,
                rows: self.rows(),
                cols: self.cols(),
            }
            .serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Nonogram {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let SerdeNonogram {
                rows,
                cols,
                fills,
                rules,
                colors,
            } = SerdeNonogram::deserialize(deserializer)?;

            let rules = Rules::from_serde(rules, rows, cols);
            let nonogram = Self {
                fills,
                rules,
                colors,
            };

            Ok(nonogram)
        }
    }
}
