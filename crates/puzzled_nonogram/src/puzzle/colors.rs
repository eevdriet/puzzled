use std::collections::{
    BTreeMap,
    btree_map::{Iter, Keys, Values},
};

use puzzled_core::Color;

use crate::{ColorId, Fill};

#[derive(Debug, Default, Clone)]
pub struct Colors(pub(crate) BTreeMap<ColorId, Color>);

impl Colors {
    pub fn new(colors: BTreeMap<ColorId, Color>) -> Self {
        Self(colors)
    }

    pub fn insert_fill(&mut self, fill: Fill, color: Color) -> Option<Color> {
        match fill {
            Fill::Color(id) => self.0.insert(id, color),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, id: ColorId, color: Color) -> Option<Color> {
        self.0.insert(id, color)
    }

    pub fn iter(&self) -> Iter<'_, ColorId, Color> {
        self.0.iter()
    }

    pub fn keys(&self) -> Keys<'_, ColorId, Color> {
        self.0.keys()
    }

    pub fn values(&self) -> Values<'_, ColorId, Color> {
        self.0.values()
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use std::collections::BTreeMap;

    use puzzled_core::{Color, ColorId};
    use serde::{Deserialize, Serialize};

    use crate::Colors;

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
            let colors = BTreeMap::<ColorId, Color>::deserialize(deserializer)?;
            let colors = Colors(colors);

            Ok(colors)
        }
    }
}
