mod iter;

pub use iter::*;

use std::fmt::Debug;

use crate::Fill;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Run {
    pub fill: Fill,
    pub count: usize,
}

impl Run {
    pub fn new(fill: Fill, count: usize) -> Self {
        Self { fill, count }
    }
}

impl From<(Fill, usize)> for Run {
    fn from((fill, count): (Fill, usize)) -> Self {
        Self { fill, count }
    }
}
impl From<Run> for (Fill, usize) {
    fn from(run: Run) -> Self {
        (run.fill, run.count)
    }
}

impl Debug for Run {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.fill.key(None).unwrap_or('?');

        write!(f, "({key}, {})", self.count)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use crate::{Fill, Run};

    #[derive(Serialize, Deserialize)]
    struct SerdeRun {
        fill: Fill,
        count: usize,
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Run {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            SerdeRun {
                fill: self.fill,
                count: self.count,
            }
            .serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Run {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let SerdeRun { fill, count } = SerdeRun::deserialize(deserializer)?;
            let run = Run { fill, count };

            Ok(run)
        }
    }
}
