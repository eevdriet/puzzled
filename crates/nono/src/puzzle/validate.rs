use crate::{Fill, Line, LinePosition, Puzzle, Rule};

#[derive(Debug)]
pub enum LineValidation {
    /// All cells in the line are validated by the rule
    Valid,

    /// All cells in the line are validated and solve the rule
    Solved,

    /// Line is validated by a rule of different length
    LengthMismatch { rule_len: u16, line_len: u16 },

    /// Line includes a fill that is not include in the rule
    InvalidFill(Fill),

    /// Some cells in the line are invalidated by the rule
    Invalid,
}

impl LineValidation {
    pub fn symbol(&self) -> char {
        match self {
            LineValidation::Invalid
            | LineValidation::LengthMismatch { .. }
            | LineValidation::InvalidFill(..) => '⛌',
            LineValidation::Solved => '✓',
            _ => ' ',
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(self, LineValidation::Valid | LineValidation::Solved)
    }
}

impl Puzzle {
    pub fn validate(&self, rule: &Rule, line: Line) -> LineValidation {
        // Make sure the rule length is valid
        let rule_len = rule.line_len();
        let line_len = self.line_len(line);

        if rule_len != line_len {
            tracing::warn!(
                "Tried to apply {rule:?} to {line:?}, but found length mismatch ({rule_len}, {line_len})",
            );

            return LineValidation::LengthMismatch { rule_len, line_len };
        }

        // Then validate whether the line can satisfy the rule
        // if !self.validate_dp(rule, line) {
        //     return LineValidation::Invalid;
        // }

        // If so, check if it solve the rule
        // self.validate_iter(rule, line)

        LineValidation::Valid
    }

    fn validate_iter(&self, rule: &Rule, line: Line) -> LineValidation {
        let rule_iter = rule.runs().iter();
        let line_iter = self.iter_runs(line);

        if rule_iter.clone().count() != line_iter.clone().count() {
            return LineValidation::Valid;
        }

        if rule_iter.zip(line_iter).all(|(run1, run2)| *run1 == run2) {
            LineValidation::Solved
        } else {
            LineValidation::Valid
        }

        // for (idx, run) in self.iter_runs(line).enumerate() {
        //     let Some(rule_run) = current_rule else {
        //         tracing::info!("Extra run including in puzzle but not in rule");
        //         return LineValidation::Invalid;
        //     };
        //
        //     // Fill mismatch
        //     if run.fill != rule_run.fill {
        //         tracing::info!(
        //             "Fills differ for the {}th run: {:?} (puzzle) v.s. {:?} (rule)",
        //             idx + 1,
        //             run.fill,
        //             rule_run.fill
        //         );
        //         return LineValidation::Invalid;
        //     }
        //
        //     // Run too long
        //     if run.count > rule_run.count {
        //         tracing::info!(
        //             "Counts differ for the {}th run: {:?} (puzzle) v.s. {:?}",
        //             idx + 1,
        //             run.count,
        //             rule_run.count
        //         );
        //         return LineValidation::Invalid;
        //     }
        //
        //     // If we exactly matched this rule run, advance
        //     if run.count == rule_run.count {
        //         current_rule = rule_iter.next();
        //     } else {
        //         // Partial run → must still be extending same rule run
        //         // BUT we must ensure puzzle didn't insert blanks splitting it
        //         // That requires checking adjacency in the grid — not here
        //     }
        // }
        //
        // LineValidation::Valid
    }

    fn validate_dp(&self, rule: &Rule, line: Line) -> bool {
        let runs = rule.runs();
        let n = self.line_len(line) as usize;
        let m = runs.len();

        // dp[offset][r]: first offset cells can fit the first r runs
        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[0][0] = true;

        for offset in 0..=n {
            let pos = LinePosition::new(line, offset as u16);

            #[allow(clippy::needless_range_loop)]
            for r in 0..=m {
                // Cannot fit another run if previous runs already do not fit
                if !dp[offset][r] {
                    continue;
                }

                // Option 1: skip next position
                if offset < n && matches!(self[pos], Fill::Cross | Fill::Blank) {
                    dp[offset + 1][r] = true;
                }

                // Option 2: fill next position with the next run

                // Already considered all runs
                if r >= m {
                    continue;
                }

                // Determine the offset of the next run
                let run = &runs[r];
                let len = run.count;

                let next_pos = pos + len;
                let next_offset = next_pos.offset as usize;

                // Run cannot fit in the remaining space
                if next_offset > n {
                    continue;
                }

                // Verify that all spaces in the run are empty or filled with the correct color
                // If not, continue trying the next run from this position
                let mut ok = true;

                for idx in 0..len {
                    match self[pos + idx] {
                        Fill::Cross => {
                            ok = false;
                            break;
                        }
                        col @ Fill::Color(_) if col != run.fill => {
                            ok = false;
                            break;
                        }
                        _ => {}
                    }
                }

                if !ok {
                    continue;
                }

                // Make sure to leave a space between runs with the same (colored) fill
                if next_offset < n
                    && matches!(self[next_pos], col @Fill::Color(_) if col == run.fill)
                {
                    continue;
                }

                // Next run fits in the spaces right after
                dp[next_offset][r + 1] = true;
            }
        }

        dp[n][m]
    }

    fn validate_masks(&self, rule: &Rule, line: Line) -> LineValidation {
        // Verify each of the fills in the line that are currently set
        // Note the .filter to avoid fills that have been previously been set but not currently
        let Some(masks) = self.masks.get(&line) else {
            return LineValidation::Valid;
        };

        for (&fill, mask) in masks.iter().filter(|(_, mask)| mask.any()) {
            let set = mask.clone();

            // Invalidate right away if rule doesn't include fill
            let Some(required) = rule.fill_constraint(fill) else {
                return LineValidation::InvalidFill(fill);
            };

            // Make sure that all cells that are required are set
            let required_unset = required & !set;
            if required_unset.any() {
                return LineValidation::InvalidFill(fill);
            }
        }

        LineValidation::Valid
    }

    fn validate_fill(&self, _rule: &Rule, _line: Line) -> LineValidation {
        LineValidation::Valid
    }
}
