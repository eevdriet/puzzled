mod cell;
mod colors;
mod fill;
mod find;
mod rule;
mod run;

use derive_more::{Index, IndexMut};
use puzzled_core::{Grid, Metadata, Puzzle};

pub use cell::*;
pub use colors::*;
pub use fill::*;
pub use find::*;
pub use rule::*;
pub use run::*;

#[derive(Debug, Index, IndexMut)]
pub struct Nonogram {
    #[index]
    #[index_mut]
    fills: Fills,

    rules: Rules,
    colors: Colors,

    meta: Metadata,
}

impl Puzzle for Nonogram {
    type Solution = Grid<Fill>;
}

impl Nonogram {
    pub fn new(fills: Fills, colors: Colors, meta: Metadata) -> Self {
        let rules = Rules::from_fills(&fills);

        Self {
            fills,
            rules,
            colors,
            meta,
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

    pub fn meta(&self) -> &Metadata {
        &self.meta
    }
}

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
