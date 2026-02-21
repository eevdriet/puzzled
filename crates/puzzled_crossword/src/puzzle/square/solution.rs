use std::fmt;

/// Solution to a [square](crate::Square) that can be used to verify its correctness
///
/// In almost all cases, solutions consist of a single [letter](Self::Letter).
/// However, users may define a [rebus](Self::Rebus) to construct a multi-letter solution.
/// In `*.puz` files, rebuses are defined from the [GRBS and RTBL sections](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Solution {
    /// One-letter solution
    Letter(char),

    /// Multiple-letter solution, a.k.a. a rebus
    Rebus(String),
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Letter(letter) => write!(f, "{letter}"),
            Self::Rebus(rebus) => write!(f, "{rebus}"),
        }
    }
}

impl From<String> for Solution {
    fn from(value: String) -> Self {
        match value.len() {
            1 => {
                let letter = value.chars().next().expect("Verified non-zero length");
                Solution::Letter(letter)
            }
            _ => Solution::Rebus(value),
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::Solution;

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Solution {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut buf = [0; 4];

            serializer.serialize_str(match self {
                Solution::Letter(letter) => letter.encode_utf8(&mut buf),
                Solution::Rebus(rebus) => rebus,
            })
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Solution {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Ok(String::deserialize(deserializer)?.into())
        }
    }
}
