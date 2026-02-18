use crate::{Fill, Run};

#[derive(Debug, Clone)]
pub struct Runs<I>
where
    I: Iterator,
{
    iter: I,
    curr: Option<Run>,

    skip_non_colored: bool,
    finished: bool,
}

impl<I> Runs<I>
where
    I: Iterator,
    I::Item: Into<Fill>,
{
    pub fn new(mut iter: I, skip_non_colored: bool) -> Self {
        let curr = iter.next().map(|fill| Run {
            fill: fill.into(),
            count: 1,
        });

        Self {
            iter,
            curr,
            skip_non_colored,
            finished: false,
        }
    }
}

impl<I> Iterator for Runs<I>
where
    I: Iterator,
    I::Item: Into<Fill>,
{
    type Item = Run;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let mut curr = self.curr.take()?;
        let should_yield =
            |curr: Run| !self.skip_non_colored || matches!(curr.fill, Fill::Color(_));

        while let Some(fill) = self.iter.next() {
            let fill: Fill = fill.into();

            // Continue the current run
            if fill == curr.fill {
                curr.count += 1;
            } else {
                // Start a new run from the next fill
                self.curr = Some(Run { fill, count: 1 });

                // Yield a colored run..
                if should_yield(curr) {
                    return Some(curr);
                }
                // .. or skip to the next one
                else {
                    return self.next();
                }
            }
        }

        // Out of fills -> finish last colored run
        self.finished = true;

        should_yield(curr).then_some(curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const B: Fill = Fill::Blank;
    const X: Fill = Fill::Cross;
    const C: Fill = Fill::Color(1);
    const C2: Fill = Fill::Color(2);

    #[rstest]
    #[case(vec![B], vec![(B, 1)])]
    #[case(vec![B, B], vec![(B, 2)])]
    #[case(vec![B, B, X, B], vec![(B, 2), (X, 1), (B, 1)])]
    #[case(vec![C, X, X, C], vec![(C, 1), (X, 2), (C, 1)])]
    fn all_runs(#[case] fills: Vec<Fill>, #[case] runs: Vec<(Fill, u16)>) {
        let fill_runs: Vec<_> = Runs::new(fills.into_iter(), false).collect();
        let runs: Vec<Run> = runs.iter().map(|&val| val.into()).collect();

        assert_eq!(fill_runs, runs);
    }

    #[rstest]
    #[case(vec![B], vec![])]
    #[case(vec![B, X], vec![])]
    #[case(vec![B, X, C], vec![(C, 1)])]
    #[case(vec![C, X, C], vec![(C, 1), (C, 1)])]
    #[case(vec![C, C, C2, C, C2, C2, C, C], vec![(C, 2), (C2, 1), (C, 1), (C2, 2), (C, 2)])]
    fn colored_runs(#[case] fills: Vec<Fill>, #[case] runs: Vec<(Fill, u16)>) {
        let fill_runs: Vec<_> = Runs::new(fills.into_iter(), true).collect();
        let runs: Vec<Run> = runs.iter().map(|&val| val.into()).collect();

        assert_eq!(fill_runs, runs);
    }
}
