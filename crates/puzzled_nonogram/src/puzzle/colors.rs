use std::{collections::BTreeMap, fmt};

use derive_more::{Deref, DerefMut};
use puzzled_core::Color;

use crate::Fill;

#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Colors(pub(crate) BTreeMap<Fill, Color>);

impl Colors {
    pub fn new(mut colors: BTreeMap<Fill, Color>) -> Self {
        // Add default colors for blanks and crosses
        let black = Color::rgba(0, 0, 0, 255);

        colors.entry(Fill::Cross).or_insert(black);

        Self(colors)
    }

    pub fn prev(&self, fill: Fill) -> Option<Fill> {
        let Fill::Color(_) = fill else {
            return None;
        };

        self.0.range(..fill).next_back().map(|(prev, _)| *prev)
    }

    pub fn next(&self, fill: Fill) -> Option<Fill> {
        let Fill::Color(id) = fill else {
            return None;
        };
        let next = Fill::Color(id + 1);

        self.0.range(next..).next().map(|(next, _)| *next)
    }
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (id, color) in self.iter() {
            writeln!(f, "{id}: {color}")?;
        }

        Ok(())
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use std::collections::BTreeMap;

    use puzzled_core::Color;
    use serde::{Deserialize, Serialize};

    use crate::{Colors, Fill};

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Colors {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Colors {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let colors = BTreeMap::<Fill, Color>::deserialize(deserializer)?;
            let colors = Colors(colors);

            Ok(colors)
        }
    }
}
