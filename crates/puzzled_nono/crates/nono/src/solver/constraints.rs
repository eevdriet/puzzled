use std::collections::{HashMap, hash_map::Entry};

use bitvec::prelude::*;

use crate::{Fill, Line, LineMask, Run, Solver};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineConstraint {
    pub required: LineMask,
    pub optional: LineMask,
}

impl Solver {
    pub fn generate_rule_constraints(&mut self, line: Line) {
        // Find the rule to generate constraints for
        let Some(rule) = self.rules.get(&line) else {
            tracing::warn!("No rule exists that matches {line:?} to generate constraints for");
            return;
        };

        let runs = rule.runs();
        let line_len = rule.line_len() as usize;

        let left = fit_forwards(runs, line_len);
        let right = fit_backwards(runs, line_len);

        let mut required_cross = bitvec![1; line_len];
        let mut optional_cross = bitvec![1; line_len];

        // Generate the constraints if they do not yet exists, otherwise return early
        let constraints = match self.constraints.entry(Line::Row(0)) {
            Entry::Occupied(_) => return,
            Entry::Vacant(v) => v.insert(HashMap::new()),
        };

        for color in rule.iter_colors() {
            // Find all cells that must and may be filled for a given color
            let (required, optional) = find_filled(runs, line_len, color, &left, &right);

            // Eliminate cells that may be filled for all crossed out cells
            required_cross &= !optional.clone();
            optional_cross &= !required.clone();

            // Register the must be filled cells for the color
            let constraint = LineConstraint { required, optional };
            tracing::info!("{constraint:?} created for {color:?} on {line:?}");

            constraints.insert(color, constraint);
        }

        // Finally register the must be crossed out cells
        let constraint = LineConstraint {
            required: required_cross,
            optional: optional_cross,
        };

        let cross = Fill::Cross;
        tracing::info!("{constraint:?} created for {cross:?} on {line:?}");

        constraints.insert(cross, constraint);
    }
}

fn find_filled(
    runs: &[Run],
    line_len: usize,
    color: Fill,
    prefixes: &[Vec<bool>],
    suffixes: &[Vec<bool>],
) -> (LineMask, LineMask) {
    let m = runs.len();
    let n = line_len;

    // Find cells that may be filled by checking for each position
    // if the previous/next fills are a valid prefix/suffix
    let mut must_be_filled = bitvec![0; n];
    let mut maybe_filled = bitvec![0; n];

    for r in 0..m {
        let run = runs[r];

        // Ignore runs that we're not finding the mask for
        if run.fill != color {
            continue;
        }

        let len = run.count as usize;
        let mut run_must = bitvec![1; n];
        let mut has_any = false;

        for start in 0..=n.saturating_sub(len) {
            // Determine where the placed run must end (include gap for same fill)
            let mut end = start + len;

            if r + 1 < m && runs[r + 1].fill == run.fill {
                end += 1;
            }

            // Set all cells in the placement for a valid prefix/suffix pair
            if prefixes[r][start] && suffixes[r + 1][end.min(n)] {
                has_any = true;

                let mut mask = bitvec![0; n];
                for idx in start..start + len {
                    mask.set(idx, true);
                }

                maybe_filled |= &mask;
                run_must &= &mask;
            }
        }

        if has_any {
            must_be_filled |= run_must;
        }
    }

    (must_be_filled, maybe_filled)
}

/// Fit runs in a line going forwards
///
/// * `runs`: Runs to fit in the line
/// * `line_len`: Length of the line to fit runs in
fn fit_forwards(runs: &[Run], line_len: usize) -> Vec<Vec<bool>> {
    let m = runs.len();
    let n = line_len;

    // dp[offset][r]: runs[0..r] fit in cells[0..offset]
    let mut dp = vec![vec![false; n + 1]; m + 1];

    // No runs can always fit
    dp[0][0] = true;

    for offset in 0..=n {
        for r in 0..=m {
            // Cannot fit another run if previous runs already do not fit
            if !dp[r][offset] {
                continue;
            }

            // Option 1: leave cell empty
            if offset < n {
                dp[r][offset + 1] = true;
            }

            // Already considered all runs
            if r >= m {
                continue;
            }

            // Option 2: place run r
            let len = runs[r].count as usize;
            let mut next = offset + len;

            // Leave a gap for adjacent runs of the same fill
            if r + 1 < m && runs[r].fill == runs[r + 1].fill {
                next += 1;
            }

            if next <= n {
                dp[r + 1][next] = true;
            }
        }
    }

    dp
}

/// Fit runs in a line going backwards
///
/// * `runs`: Runs to fit in the line
/// * `line_len`: Length of the line to fit runs in
fn fit_backwards(runs: &[Run], line_len: usize) -> Vec<Vec<bool>> {
    let m = runs.len();
    let n = line_len;

    // dp[r][offset]: runs[r..m] fit in cells[offset..n]
    let mut dp = vec![vec![false; n + 1]; m + 1];

    // No runs can always fit
    dp[m][n] = true;

    for offset in (0..n).rev() {
        for r in 0..=m {
            // Another run fits if previous runs already do not fit
            if dp[r][offset + 1] {
                // Option 1: leave cell empty
                dp[r][offset] = true;
            }

            // Already considered all runs
            if r == m {
                continue;
            }

            // Option 2: place run r
            let len = runs[r].count as usize;
            let mut next = offset + len;

            // Leave a gap for adjacent runs of the same fill
            if r + 1 < m && runs[r].fill == runs[r + 1].fill {
                next += 1;
            }

            if next <= n && dp[r + 1][next] {
                dp[r][offset] = true;
            }
        }
    }

    dp
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Fill;
    use rstest::rstest;
    use tracing_test::traced_test;

    const B: Fill = Fill::Blank;
    const X: Fill = Fill::Cross;
    const C: Fill = Fill::Color(1);
    const C2: Fill = Fill::Color(2);

    fn ft_vec(len: usize, false_until_idx: usize) -> Vec<bool> {
        (0..len).map(|idx| idx >= false_until_idx).collect()
    }

    fn tf_vec(len: usize, true_until_idx: usize) -> Vec<bool> {
        (0..len).map(|idx| idx < true_until_idx).collect()
    }

    #[rstest]
    #[case::single_run(vec![(C,1)], 1, vec![ft_vec(2, 0), ft_vec(2, 1)])]
    #[case::single_run(vec![(C,1)], 2, vec![ft_vec(3, 0), ft_vec(3, 1)])]
    #[case::single_run(vec![(C,1)], 3, vec![ft_vec(4, 0), ft_vec(4, 1)])]
    #[case::single_run(vec![(C,2)], 2, vec![ft_vec(3, 0), ft_vec(3, 2)])]
    #[case::single_run(vec![(C,2)], 3, vec![ft_vec(4, 0), ft_vec(4, 2)])]
    #[case::single_run(vec![(C,2)], 4, vec![ft_vec(5, 0), ft_vec(5, 2)])]
    #[case::single_run(vec![(C,3)], 3, vec![ft_vec(4, 0), ft_vec(4, 3)])]
    #[case::single_run(vec![(C,3)], 4, vec![ft_vec(5, 0), ft_vec(5, 3)])]
    #[case::single_run(vec![(C,3)], 5, vec![ft_vec(6, 0), ft_vec(6, 3)])]
    #[case::multiple_runs(vec![(C,1), (C,1)], 3, vec![ft_vec(4, 0), ft_vec(4, 2), ft_vec(4, 3)])]
    #[case::multiple_runs(vec![(C,1), (C,1)], 4, vec![ft_vec(5, 0), ft_vec(5, 2), ft_vec(5, 3)])]
    #[case::multiple_runs(vec![(C,3), (C,2), (C,1)], 8, vec![ft_vec(9, 0), ft_vec(9, 4), ft_vec(9, 7), ft_vec(9, 8)])]
    #[case::mixed_runs(vec![(C,1), (C,1)], 3, vec![ft_vec(4, 0), ft_vec(4, 2), ft_vec(4, 3)])]
    fn test_fit_forwards(
        #[case] runs: Vec<(Fill, u16)>,
        #[case] line_len: usize,
        #[case] expected: Vec<Vec<bool>>,
    ) {
        let runs: Vec<Run> = runs.iter().map(|&val| val.into()).collect();
        let dp = fit_forwards(&runs, line_len);

        assert_eq!(dp, expected);
    }

    #[rstest]
    #[case::single_run(vec![(C,1)], 1, vec![tf_vec(2, 1), tf_vec(2, 2)])]
    #[case::single_run(vec![(C,1)], 2, vec![tf_vec(3, 2), tf_vec(3, 3)])]
    #[case::single_run(vec![(C,1)], 3, vec![tf_vec(4, 3), tf_vec(4, 4)])]
    #[case::single_run(vec![(C,2)], 2, vec![tf_vec(3, 1), tf_vec(3, 3)])]
    #[case::single_run(vec![(C,2)], 3, vec![tf_vec(4, 2), tf_vec(4, 4)])]
    #[case::single_run(vec![(C,2)], 4, vec![tf_vec(5, 3), tf_vec(5, 5)])]
    #[case::single_run(vec![(C,3)], 3, vec![tf_vec(4, 1), tf_vec(4, 4)])]
    #[case::single_run(vec![(C,3)], 4, vec![tf_vec(5, 2), tf_vec(5, 5)])]
    #[case::single_run(vec![(C,3)], 5, vec![tf_vec(6, 3), tf_vec(6, 6)])]
    #[case::multiple_runs(vec![(C,1), (C,1)], 3, vec![tf_vec(4, 1), tf_vec(4, 3), tf_vec(4, 4)])]
    #[case::multiple_runs(vec![(C,1), (C,1)], 4, vec![tf_vec(5, 2), tf_vec(5, 4), tf_vec(5, 5)])]
    fn test_fit_backwards(
        #[case] runs: Vec<(Fill, u16)>,
        #[case] line_len: usize,
        #[case] expected: Vec<Vec<bool>>,
    ) {
        let runs: Vec<Run> = runs.iter().map(|&val| val.into()).collect();
        let dp = fit_backwards(&runs, line_len);

        assert_eq!(dp, expected);
    }

    #[traced_test]
    #[rstest]
    #[case::single_run(vec![(C,1)], 1, bitvec![1])]
    #[case::single_run(vec![(C,1)], 2, bitvec![0, 0])]
    #[case::single_run(vec![(C,1)], 4, bitvec![0, 0, 0, 0])]
    #[case::single_run(vec![(C,4)], 6, bitvec![0, 0, 1, 1, 0, 0])]
    #[case::exact_fit(vec![(C,1), (C, 1)], 3, bitvec![1, 0, 1])]
    #[case::exact_fit(vec![(C,2), (C, 1), (C, 3)], 8, bitvec![1, 1, 0, 1, 0, 1, 1, 1])]
    #[case::exact_fit(vec![(C,2), (C2, 1), (C2, 3)], 8, bitvec![0, 1, 0, 0, 0, 0, 0, 0])]
    fn test_must_be_filled(
        #[case] runs: Vec<(Fill, u16)>,
        #[case] line_len: usize,
        #[case] expected: BitVec,
    ) {
        let runs: Vec<Run> = runs.iter().map(|&val| val.into()).collect();

        let prefixes = fit_forwards(&runs, line_len);
        let suffixes = fit_backwards(&runs, line_len);
        let (mask, _) = find_filled(&runs, line_len, C, &prefixes, &suffixes);

        assert_eq!(mask, expected);
    }

    #[traced_test]
    #[rstest]
    #[case::single_run(vec![(C,1)], 1, bitvec![1])]
    #[case::single_run(vec![(C,1)], 2, bitvec![1, 1])]
    #[case::single_run(vec![(C,1)], 4, bitvec![1, 1, 1, 1])]
    #[case::exact_fit(vec![(C,1), (C, 1)], 3, bitvec![1, 0, 1])]
    #[case::exact_fit(vec![(C,2), (C, 1), (C, 3)], 8, bitvec![1, 1, 0, 1, 0, 1, 1, 1])]
    #[case::exact_fit(vec![(C,2), (C2, 1), (C2, 3)], 8, bitvec![1, 1, 1, 0, 0, 0, 0, 0])]
    fn test_maybe_filled(
        #[case] runs: Vec<(Fill, u16)>,
        #[case] line_len: usize,
        #[case] expected: BitVec,
    ) {
        let runs: Vec<Run> = runs.iter().map(|&val| val.into()).collect();

        let prefixes = fit_forwards(&runs, line_len);
        let suffixes = fit_backwards(&runs, line_len);
        let (_, mask) = find_filled(&runs, line_len, C, &prefixes, &suffixes);

        assert_eq!(mask, expected);
    }
}
