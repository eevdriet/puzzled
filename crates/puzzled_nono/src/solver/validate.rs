use crate::{Fill, Line, LineConstraint, LinePosition, Puzzle, Rule, Solver};

#[derive(Debug, Clone, Copy)]
pub enum LineValidation {
    /// All cells in the line are validated by the rule
    Valid,

    /// All cells in the line are validated and solve the rule
    Solved,

    MissingRule(Line),

    /// Line is validated by a rule of different length
    LengthMismatch {
        rule_len: u16,
        line_len: u16,
    },

    /// Line includes a fill that is not include in the rule
    InvalidFill(Fill),

    /// Some cells in the line are invalidated by the rule
    Invalid,
}

impl LineValidation {
    pub fn symbol(&self) -> char {
        match self {
            LineValidation::Solved => '✓',
            v if !v.is_valid() => '⛌',
            _ => ' ',
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(self, LineValidation::Valid | LineValidation::Solved)
    }
}

impl Solver {
    pub fn validate(&mut self, puzzle: &Puzzle, line: Line) -> LineValidation {
        let Some(rule) = self.rules.get(&line) else {
            tracing::warn!("No rule exists that matches {line:?} to generate constraints for");
            return LineValidation::MissingRule(line);
        };

        // Make sure the rule length is valid
        let rule_len = rule.line_len();
        let line_len = puzzle.line_len(line);

        if rule_len != line_len {
            tracing::warn!(
                "Tried to apply {rule:?} to {line:?}, but found length mismatch ({rule_len}, {line_len})",
            );

            return LineValidation::LengthMismatch { rule_len, line_len };
        }

        // Then do a quick validation with the rule masks
        let validation = self.validate_masks(line);
        if !validation.is_valid() {
            return validation;
        }

        // If still valid, validate with a DP
        let validation = self.validate_dp(puzzle, rule, line);
        if !validation.is_valid() {
            return validation;
        }

        // If so, check if it solve the rule
        self.validate_iter(puzzle, rule, line)
    }

    fn validate_iter(&self, puzzle: &Puzzle, rule: &Rule, line: Line) -> LineValidation {
        let rule_iter = rule.runs().iter();
        let line_iter = puzzle.iter_runs(line);

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

    fn validate_dp(&self, puzzle: &Puzzle, rule: &Rule, line: Line) -> LineValidation {
        let runs = rule.runs();
        let n = puzzle.line_len(line) as usize;
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
                if offset < n && matches!(puzzle[pos], Fill::Cross | Fill::Blank) {
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
                    match puzzle[pos + idx] {
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
                    && matches!(puzzle[next_pos], col @Fill::Color(_) if col == run.fill)
                {
                    continue;
                }

                // Next run fits in the spaces right after
                dp[next_offset][r + 1] = true;
            }
        }

        if dp[n][m] {
            LineValidation::Valid
        } else {
            LineValidation::Invalid
        }
    }

    fn validate_masks(&self, line: Line) -> LineValidation {
        // Get the puzzle masks for the current
        // If none are set (empty line), it is always valid
        let Some(masks) = self.masks.get(&line) else {
            return LineValidation::Valid;
        };

        // Get the line constraints for the current rule
        // If it is unconstrained, the line is always valid
        let Some(constraints) = self.constraints.get(&line) else {
            return LineValidation::Solved;
        };

        tracing::info!("Validate {line:?}");
        tracing::info!("\tMasks: {masks:?}");
        tracing::info!("\tConstraints: {constraints:?}");

        // Verify each of the fills in the line that are currently set
        // Note the .filter to avoid fills that have been previously been set but not currently
        for (&fill, mask) in masks.iter().filter(|(_, mask)| mask.any()) {
            // Invalidate right away if rule doesn't include current fill
            let Some(LineConstraint { required, optional }) = constraints.get(&fill) else {
                tracing::info!("Constraint not found for {fill:?} on {line:?}");
                return LineValidation::InvalidFill(fill);
            };

            // Fill is invalid if it's not placed on one of the optional cells
            if !(optional.clone() & mask).any() {
                tracing::info!("Invalid fill for {line:?}");
                tracing::info!("\tRequired bits: {required}");
                tracing::info!("\tOptional bits: {optional}");
                tracing::info!("\tSet:           {mask}");

                return LineValidation::InvalidFill(fill);
            }
        }

        LineValidation::Valid
    }

    fn validate_fill(&self, _rule: &Rule, _line: Line) -> LineValidation {
        LineValidation::Valid
    }
}
