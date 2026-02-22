mod colors;
mod error;
mod fill;
mod find;
mod macros;
mod rule;
mod run;

use bitvec::vec::BitVec;
use derive_more::{Index, IndexMut};
use puzzled_core::{Grid, Line, Metadata, add_metadata};
use std::collections::HashMap;

pub use colors::*;
pub use error::*;
pub use fill::*;
pub use find::*;
pub use rule::*;
pub use run::*;

#[derive(Debug, Default, Index, IndexMut)]
pub struct Nonogram {
    #[index]
    #[index_mut]
    fills: Fills,

    rules: Rules,
    colors: Colors,

    meta: Metadata,
}

impl Nonogram {
    pub fn new(fills: Fills, rules: Rules, colors: Colors, meta: Metadata) -> Self {
        Self {
            fills,
            rules,
            colors,
            meta,
        }
    }

    pub fn new_empty(rules: Rules, colors: Colors, meta: Metadata) -> Option<Self> {
        // Start with a blank grid of the same dimensions as the rules
        let grid = Grid::new_from(rules.rows.len(), rules.cols.len(), Fill::Blank)?;

        let fills = Fills::new(grid);
        let nonogram = Self {
            fills,
            rules,
            colors,
            meta,
        };

        Some(nonogram)
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

    pub fn colors(&self) -> &Colors {
        &self.colors
    }

    pub fn colors_mut(&mut self) -> &mut Colors {
        &mut self.colors
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

add_metadata!(Nonogram);

#[cfg(feature = "serde")]
mod serde_impl {
    use puzzled_core::Metadata;
    use serde::{Deserialize, Serialize};

    use crate::{Colors, Fills, Nonogram, Rules, SerdeRules};

    #[derive(Serialize, Deserialize)]
    struct SerdeNonogram {
        rows: usize,
        cols: usize,

        fills: Fills,
        rules: SerdeRules,
        colors: Colors,
        meta: Metadata,
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
            let meta = self.meta.clone();

            SerdeNonogram {
                rules,
                fills,
                colors,
                meta,
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
                meta,
            } = SerdeNonogram::deserialize(deserializer)?;

            let rules = Rules::from_serde(rules, rows, cols);
            let nonogram = Self {
                fills,
                rules,
                colors,
                meta,
            };

            Ok(nonogram)
        }
    }
}
