mod cells;

pub use cells::*;

use derive_more::{Deref, DerefMut};
use puzzled_core::Cell;

use crate::Fill;

#[derive(Deref, Debug, DerefMut, PartialEq, Eq, Clone)]
pub struct NonogramCell(pub(crate) Cell<Fill>);

impl NonogramCell {
    pub fn new(cell: Cell<Fill>) -> Self {
        Self(cell)
    }

    pub fn fill(&self) -> Fill {
        self.solution.unwrap_or_default()
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use puzzled_core::Entry;
    use serde::{Deserialize, Serialize};

    use crate::{Fill, NonogramCell};

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for NonogramCell {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for NonogramCell {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let cell = Entry::<Fill>::deserialize(deserializer)?;
            let cell = NonogramCell::new(cell);

            Ok(cell)
        }
    }
}
