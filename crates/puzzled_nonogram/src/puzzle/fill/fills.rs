use derive_more::{Deref, DerefMut, Index, IndexMut};
use puzzled_core::{Grid, Line, LineIter};

use crate::{Fill, Runs};

#[derive(Debug, Default, PartialEq, Eq, Deref, DerefMut, Clone, Index, IndexMut)]
pub struct Fills(Grid<Fill>);

impl Fills {
    pub fn new(fills: Grid<Fill>) -> Self {
        Self(fills)
    }

    pub fn iter_line_runs<'a>(&'a self, line: Line) -> Runs<LineIter<'a, Fill>> {
        let fills = self.iter_line(line);
        Runs::new(fills, true)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use puzzled_core::Grid;
    use serde::{Deserialize, Serialize};

    use crate::{Fill, Fills};

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Fills {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Fills {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let grid = Grid::<Fill>::deserialize(deserializer)?;
            let fills = Fills::new(grid);

            Ok(fills)
        }
    }
}
