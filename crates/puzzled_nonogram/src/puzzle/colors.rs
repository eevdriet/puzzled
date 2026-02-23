use std::collections::BTreeMap;

use derive_more::{Deref, DerefMut};
use puzzled_core::Color;

use crate::Fill;

#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Colors(pub(crate) BTreeMap<Fill, Color>);

impl Colors {
    pub fn new(mut colors: BTreeMap<Fill, Color>) -> Self {
        // Add default colors for blanks and crosses
        let white = Color::rgba(255, 255, 255, 255);

        colors.entry(Fill::Blank).or_insert(white);
        colors.entry(Fill::Cross).or_insert(white);

        Self(colors)
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
