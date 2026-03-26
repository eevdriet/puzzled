use std::{fmt, str::FromStr};

use derive_more::{Deref, DerefMut};
use puzzled_core::Word;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut)]
pub struct Skyscraper(usize);

impl Skyscraper {
    pub fn new(height: usize) -> Self {
        Self(height)
    }

    pub fn height(&self) -> usize {
        self.0
    }
}

impl fmt::Display for Skyscraper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Skyscraper {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let height = usize::from_str(value).map_err(|_| ())?;
        let skyscraper = Skyscraper::new(height);

        Ok(skyscraper)
    }
}

impl Word for Skyscraper {
    fn is_word(&self) -> bool {
        true
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::Skyscraper;

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Skyscraper {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Skyscraper {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let height = usize::deserialize(deserializer)?;
            let skyscraper = Skyscraper::new(height);

            Ok(skyscraper)
        }
    }
}
