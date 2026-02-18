mod iter;

pub use iter::*;

use std::fmt::Debug;

use serde::Deserialize;

use crate::Fill;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
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
