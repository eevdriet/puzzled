mod puzzle;
mod slice;

pub use puzzle::*;
pub use slice::*;

use derive_more::Debug;

use crate::{Fill, FillMask, Run, Runs};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    runs: Vec<Run>,

    #[debug(skip)]
    fills: FillMask,

    #[debug(skip)]
    prefix_lens: Vec<u16>,
    line_len: u16,
}

impl Rule {
    pub fn new(runs: Vec<Run>, line_len: u16) -> Self {
        let mut fills = FillMask::new();
        let mut prefix_lens = Vec::with_capacity(runs.len());

        // Manually extract the first run
        let first = match runs.first() {
            None => {
                return Self {
                    runs,
                    fills,
                    line_len,
                    prefix_lens,
                };
            }
            Some(run) => run,
        };

        let mut len = first.count;
        fills.add(first.fill);
        prefix_lens.push(len);

        // Go over the runs pairwise to compare their fills
        for window in runs.windows(2) {
            let prev = window[0];
            let curr = window[1];

            // Add the run length to the running total
            len += curr.count;

            // Add space between same-fill runs
            if curr.fill == prev.fill {
                len += 1;
            }

            fills.add(curr.fill);
            prefix_lens.push(len);
        }

        Self {
            runs,
            prefix_lens,
            fills,
            line_len,
        }
    }

    pub fn from_fills<I>(fills: I) -> Self
    where
        I: IntoIterator<Item = Fill>,
    {
        let mut line_len = 0;
        let iter = fills.into_iter().inspect(|_| line_len += 1);

        let runs: Vec<_> = Runs::new(iter, true).collect();

        Self::new(runs, line_len)
    }

    // Getters
    pub fn iter_fill_runs(&self, fill: Fill) -> impl Iterator<Item = &Run> {
        self.runs.iter().filter(move |run| run.fill == fill)
    }

    pub fn iter_colors(&self) -> impl Iterator<Item = Fill> {
        self.fills.iter_colors()
    }

    pub fn line_len(&self) -> u16 {
        self.line_len
    }

    pub fn len(&self) -> u16 {
        match self.prefix_lens.len().checked_sub(1) {
            None => 0,
            Some(end) => self.prefix_lens[end],
        }
    }

    pub fn min_run(&self, pos: u16) -> u16 {
        let run_pos = match self.prefix_lens.binary_search(&pos) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };

        run_pos.clamp(0, self.runs.len().saturating_sub(1)) as u16
    }

    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }

    pub fn runs(&self) -> &Vec<Run> {
        &self.runs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use tracing_test::traced_test;

    const B: Fill = Fill::Blank;
    const X: Fill = Fill::Cross;
    const C1: Fill = Fill::Color(1);
    const C2: Fill = Fill::Color(2);

    fn fill_counts_to_runs(fill_counts: Vec<(Fill, u16)>) -> Vec<Run> {
        fill_counts
            .iter()
            .map(|&val| val.into())
            .collect::<Vec<Run>>()
    }
    fn runs_to_fills(runs: Vec<Run>) -> Vec<Fill> {
        let mut fills = vec![];

        for run in runs {
            fills.extend(vec![run.fill; run.count as usize]);
        }

        fills
    }

    #[rstest]
    #[case::single_color(vec![C1], vec![(C1, 1)])]
    #[case::single_blank(vec![B], vec![])]
    #[case::multiple(vec![B, X, C1], vec![(C1, 1)])]
    fn test_from_fills_runs(#[case] fills: Vec<Fill>, #[case] expected: Vec<(Fill, u16)>) {
        let rule = Rule::from_fills(fills.into_iter());
        let runs = fill_counts_to_runs(expected);

        assert_eq!(rule.runs, runs);
    }

    #[rstest]
    #[case::single_color(vec![C1], 1)]
    #[case::single_blank(vec![B], 1)]
    #[case::multiple(vec![B, X, C1], 3)]
    fn test_from_fills_line_len(#[case] fills: Vec<Fill>, #[case] expected: u16) {
        let rule = Rule::from_fills(fills.into_iter());

        assert_eq!(rule.line_len(), expected);
    }

    #[traced_test]
    #[rstest]
    #[case::no_space(vec![C1, C2], 2)]
    #[case::mono_color(vec![C1, C1, C1, B, C1, C1, C1, B], 7)]
    fn test_from_fills_len(#[case] fills: Vec<Fill>, #[case] expected: u16) {
        let rule = Rule::from_fills(fills.into_iter());

        assert_eq!(rule.len(), expected);
    }
}
