use crate::{Fill, Run};

#[derive(Debug, Clone)]
pub struct Runs<I> {
    iter: I,
    run: Option<Run>,

    skip_non_colored: bool,
    finished: bool,
}

impl<I> Runs<I>
where
    I: Iterator<Item = Option<Fill>>,
{
    pub fn new(iter: I, skip_non_colored: bool) -> Self {
        Self {
            iter,
            run: None,
            skip_non_colored,
            finished: false,
        }
    }
}

impl<I> Iterator for Runs<I>
where
    I: Iterator<Item = Option<Fill>>,
{
    type Item = Run;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let should_yield =
            |curr: Run| !self.skip_non_colored || matches!(curr.fill, Fill::Color(_));

        while let Some(fill) = self.iter.next() {
            // Skip over blanks
            let Some(fill) = fill else {
                continue;
            };

            // Create a run if none currently exists
            let mut run = self.run.unwrap_or(Run { fill, count: 0 });

            // Continue the current run
            if fill == run.fill {
                run.count += 1;
            } else {
                // Start a new run from the next fill
                self.run = Some(Run { fill, count: 1 });

                // Yield a colored run..
                if should_yield(run) {
                    return Some(run);
                }
                // .. or skip to the next one
                else {
                    return self.next();
                }
            }
        }

        // Out of fills -> finish last colored run
        self.finished = true;

        self.run.and_then(|run| should_yield(run).then_some(run))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const B: Option<Fill> = None;
    const X: Option<Fill> = Some(Fill::Cross);
    const C: Option<Fill> = Some(Fill::Color(1));
    const C2: Option<Fill> = Some(Fill::Color(2));

    fn fill_counts_to_runs(fill_counts: Vec<(Option<Fill>, usize)>) -> Vec<Run> {
        fill_counts
            .into_iter()
            .filter_map(|(opt_fill, count)| {
                let fill = opt_fill?;
                Some((fill, count).into())
            })
            .collect::<Vec<Run>>()
    }

    #[rstest]
    #[case(vec![B], vec![(B, 1)])]
    #[case(vec![B, B], vec![(B, 2)])]
    #[case(vec![B, B, X, B], vec![(B, 2), (X, 1), (B, 1)])]
    #[case(vec![C, X, X, C], vec![(C, 1), (X, 2), (C, 1)])]
    fn all_runs(#[case] fills: Vec<Option<Fill>>, #[case] runs: Vec<(Option<Fill>, usize)>) {
        let fill_runs: Vec<_> = Runs::new(fills.into_iter(), false).collect();
        let runs: Vec<Run> = fill_counts_to_runs(runs);

        assert_eq!(fill_runs, runs);
    }

    #[rstest]
    #[case(vec![B], vec![])]
    #[case(vec![B, X], vec![])]
    #[case(vec![B, X, C], vec![(C, 1)])]
    #[case(vec![C, X, C], vec![(C, 1), (C, 1)])]
    #[case(vec![C, C, C2, C, C2, C2, C, C], vec![(C, 2), (C2, 1), (C, 1), (C2, 2), (C, 2)])]
    fn colored_runs(#[case] fills: Vec<Option<Fill>>, #[case] runs: Vec<(Option<Fill>, usize)>) {
        let fill_runs: Vec<_> = Runs::new(fills.into_iter(), true).collect();
        let runs: Vec<Run> = fill_counts_to_runs(runs);

        assert_eq!(fill_runs, runs);
    }
}
