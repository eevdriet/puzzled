mod iter;

pub use iter::*;

use std::fmt::Debug;

use serde::Deserialize;

use crate::Fill;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Run {
    pub fill: Fill,
    pub count: u16,
}

impl Run {
    pub fn new(fill: Fill, count: u16) -> Self {
        Self { fill, count }
    }
}

impl From<(Fill, u16)> for Run {
    fn from((fill, count): (Fill, u16)) -> Self {
        Self { fill, count }
    }
}
impl From<Run> for (Fill, u16) {
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
