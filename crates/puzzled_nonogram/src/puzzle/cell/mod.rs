mod cells;

pub use cells::*;

use derive_more::{Deref, DerefMut};
use puzzled_core::{Cell, Reveal};

use crate::Fill;

#[derive(Deref, Debug, DerefMut, PartialEq, Eq, Clone)]
pub struct NonogramCell(Cell<Fill>);

impl NonogramCell {
    pub fn new(cell: Cell<Fill>) -> Self {
        Self(cell)
    }

    pub fn fill(&self) -> Fill {
        *self.entry().unwrap_or(&Fill::Blank)
    }
}

impl Reveal for NonogramCell {
    fn reveal(&mut self) {
        self.0.reveal();
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use puzzled_core::Cell;
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
            let cell = Cell::<Fill>::deserialize(deserializer)?;
            let cell = NonogramCell::new(cell);

            Ok(cell)
        }
    }
}
