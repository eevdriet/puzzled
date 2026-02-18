use std::ops::{Bound, Index, RangeBounds};

use crate::{Rule, Run};

pub struct RunsSlice<'a> {
    runs: &'a [Run],
    len: u16,
}

impl<'a> RunsSlice<'a> {
    pub fn runs(&self) -> &'a [Run] {
        self.runs
    }

    pub fn len(&self) -> u16 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }
}

impl Rule {
    pub fn slice<R>(&self, rng: R) -> RunsSlice<'_>
    where
        R: RangeBounds<u16>,
    {
        let start = match rng.start_bound() {
            Bound::Included(&idx) => idx,
            Bound::Excluded(&idx) => idx + 1,
            Bound::Unbounded => 0,
        } as usize;

        let end = match rng.end_bound() {
            Bound::Included(&idx) => idx + 1,
            Bound::Excluded(&idx) => idx,
            Bound::Unbounded => self.runs.len() as u16,
        } as usize;

        let runs = &self.runs[start..end];

        let len = if runs.is_empty() {
            0
        } else if start == 0 {
            self.prefix_lens[end - 1]
        } else {
            self.prefix_lens[end - 1] - self.prefix_lens[start]
        };

        RunsSlice { runs, len }
    }
}

impl Index<u16> for Rule {
    type Output = Run;

    fn index(&self, idx: u16) -> &Self::Output {
        &self.runs[idx as usize]
    }
}
