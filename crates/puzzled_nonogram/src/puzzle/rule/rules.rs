use std::collections::BTreeMap;

use derive_more::Deref;
use puzzled_core::{Cell, Grid, Line};

use crate::{Fill, Rule};

#[derive(Debug, thiserror::Error)]
pub enum RulesError {
    #[error("Rules contain {found} row rules, expected {expected}")]
    InvalidRowCount { found: usize, expected: usize },

    #[error("Rules contain {found} column rules, expected {expected}")]
    InvalidColCount { found: usize, expected: usize },
}

#[derive(Debug, Default, PartialEq, Eq, Deref, Clone)]
pub struct Rules {
    #[deref]
    rules: BTreeMap<Line, Rule>,

    rows: usize,
    cols: usize,
}

impl Rules {
    pub fn new(rules: BTreeMap<Line, Rule>, rows: usize, cols: usize) -> Result<Self, RulesError> {
        let row_count = rules.keys().filter(|line| line.is_row()).count();
        if row_count != rows {
            return Err(RulesError::InvalidRowCount {
                found: row_count,
                expected: rows,
            });
        }

        let col_count = rules.keys().filter(|line| line.is_col()).count();
        if col_count != cols {
            return Err(RulesError::InvalidColCount {
                found: col_count,
                expected: cols,
            });
        }

        Ok(Rules { rules, rows, cols })
    }

    pub fn new_with_default_missing(
        mut rules: BTreeMap<Line, Rule>,
        rows: usize,
        cols: usize,
    ) -> Self {
        // Add empty rules for missing rows
        for r in 0..rows {
            rules.entry(Line::Row(r)).or_default();
        }

        // Add empty rules for missing columns
        for c in 0..cols {
            rules.entry(Line::Col(c)).or_default();
        }

        Rules { rules, rows, cols }
    }

    pub fn from_fills(fills: &Grid<Cell<Fill>>) -> Self {
        let mut rules = BTreeMap::new();

        for (r, row) in fills.iter_rows().enumerate() {
            let fills = row.filter_map(|cell| cell.solution.to_owned());
            let line = Line::Row(r);

            rules.insert(line, Rule::from_fills(fills));
        }

        for (c, col) in fills.iter_cols().enumerate() {
            let fills = col.filter_map(|cell| cell.solution.to_owned());
            let line = Line::Col(c);

            rules.insert(line, Rule::from_fills(fills));
        }

        Self {
            rules,
            rows: fills.rows(),
            cols: fills.cols(),
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = (&Line, &Rule)> {
        self.iter().filter(|(line, _)| line.is_row())
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = (&Line, &Rule)> {
        self.iter().filter(|(line, _)| line.is_col())
    }

    #[cfg(feature = "serde")]
    pub(crate) fn from_serde(data: SerdeRules, rows: usize, cols: usize) -> Self {
        let rules = data
            .into_iter()
            .map(|(line, runs)| {
                let line_len = match line {
                    Line::Row(_) => cols,
                    Line::Col(_) => rows,
                };
                let rule = Rule::new(runs, line_len);

                (line, rule)
            })
            .collect();

        Self { rules, rows, cols }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn to_serde(&self) -> SerdeRules {
        self.rules
            .iter()
            .map(|(line, rule)| (*line, rule.runs.clone()))
            .collect()
    }
}

#[cfg(feature = "serde")]
pub(crate) type SerdeRules = BTreeMap<Line, crate::SerdeRule>;
