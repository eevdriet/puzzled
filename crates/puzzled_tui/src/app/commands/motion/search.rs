use std::fmt;

use derive_more::{Debug, Eq, PartialEq};
use puzzled_core::Direction;
use serde::{
    Deserialize, Deserializer,
    de::{self, IntoDeserializer, Visitor},
};

use crate::{Description, MotionBehavior};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SearchMotion {
    pub searched: Searched,

    #[partial_eq(skip)]
    #[eq(skip)]
    pub inclusive: bool,

    #[partial_eq(skip)]
    #[eq(skip)]
    pub forwards: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Searched {
    WordStart,
    WordEnd,
}

impl<'de> Deserialize<'de> for SearchMotion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SearchMotionVisitor;

        impl<'de> Visitor<'de> for SearchMotionVisitor {
            type Value = SearchMotion;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a search motion string")
            }

            fn visit_str<E>(self, motion: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Verify that a search motion is deserialized
                let mut motion = motion.strip_prefix("search_").ok_or(de::Error::custom(
                    "Search motion should be prefixed with `search_`",
                ))?;

                // Determine in which manner to search
                let mut strip_end = |pattern: &str| {
                    let prev_len = motion.len();
                    motion = motion.trim_end_matches(pattern);

                    motion.len() == prev_len
                };

                let inclusive = strip_end("_exclusive");
                let forwards = strip_end("_backwards");

                // Determine what to search based on the remaining string
                let deserializer = motion.into_deserializer();
                let searched = Searched::deserialize(deserializer)?;

                Ok(SearchMotion {
                    inclusive,
                    forwards,
                    searched,
                })
            }
        }

        deserializer.deserialize_str(SearchMotionVisitor)
    }
}

impl MotionBehavior for SearchMotion {
    fn is_mouse(&self) -> bool {
        false
    }

    fn variants() -> Vec<Self> {
        Searched::variants()
            .into_iter()
            .map(|searched| SearchMotion {
                searched,
                inclusive: true,
                forwards: true,
            })
            .collect()
    }

    fn apply_to_dir(&self, dir: Direction) -> Direction {
        if self.forwards { dir } else { !dir }
    }
}

impl MotionBehavior for Searched {
    fn variants() -> Vec<Self> {
        vec![Searched::WordStart, Searched::WordEnd]
    }
}

impl Description<()> for Searched {
    fn description(&self, _state: &()) -> Option<String> {
        let description = match self {
            Searched::WordStart => "start of the word",
            Searched::WordEnd => "end of the word",
        };

        Some(description.to_string())
    }
}

impl Description<()> for SearchMotion {
    fn description(&self, state: &()) -> Option<String> {
        let searched = self.searched.description(state)?;
        let direction_str = if self.forwards {
            "forwards"
        } else {
            "backwards"
        };
        let exclusion_str = if !self.inclusive {
            ", excluding its start/end position"
        } else {
            ""
        };

        Some(format!(
            "Find the {searched} {direction_str}{exclusion_str}",
        ))
    }
}
