use derive_more::{Deref, DerefMut, Index, IndexMut};
use puzzled_core::{ColorId, Grid, Line};

use crate::{Fill, NonogramCell, Runs};

#[derive(Debug, PartialEq, Eq, Deref, DerefMut, Clone, Index, IndexMut)]
pub struct Fills(pub(crate) Grid<NonogramCell>);

impl Fills {
    pub fn new(fills: Grid<NonogramCell>) -> Self {
        Self(fills)
    }

    pub fn iter_line_runs<'a>(
        &'a self,
        line: Line,
    ) -> Runs<impl Iterator<Item = Fill> + 'a + Clone> {
        let fills = self
            .iter_line(line)
            .filter_map(|cell| cell.solution.to_owned());

        Runs::new(fills, true)
    }

    pub fn iter_colors(&self) -> impl Iterator<Item = &Fill> {
        self.0
            .iter()
            .filter_map(|cell| match cell.solution.as_ref() {
                color @ Some(Fill::Color(_)) => color,
                _ => None,
            })
    }

    pub fn colors_ids(&self) -> Vec<ColorId> {
        let mut ids: Vec<_> = self
            .iter()
            .filter_map(|cell| match cell.solution {
                Some(Fill::Color(id)) => Some(id),
                _ => None,
            })
            .collect();

        ids.dedup();
        ids.sort();

        ids
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use puzzled_core::Grid;
    use serde::{Deserialize, Serialize};

    use crate::{Fills, NonogramCell};

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
            let grid = Grid::<NonogramCell>::deserialize(deserializer)?;
            let fills = Fills::new(grid);

            Ok(fills)
        }
    }
}
