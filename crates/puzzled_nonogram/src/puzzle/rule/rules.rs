use derive_more::Debug;
use puzzled_core::Line;

#[cfg(feature = "serde")]
use crate::Run;
use crate::{Fills, Rule};

#[derive(Debug, Default, Clone)]
pub struct Rules {
    pub rows: Vec<Rule>,
    pub cols: Vec<Rule>,
}

impl Rules {
    pub fn new(rows: Vec<Rule>, cols: Vec<Rule>) -> Self {
        Self { rows, cols }
    }

    pub fn from_fills(fills: &Fills) -> Self {
        let rows: Vec<_> = fills
            .iter_rows()
            .map(|row| {
                let fills = row.filter_map(|cell| cell.solution.to_owned());
                Rule::from_fills(fills)
            })
            .collect();

        let cols: Vec<_> = fills
            .iter_cols()
            .map(|col| {
                let fills = col.filter_map(|cell| cell.solution.to_owned());
                Rule::from_fills(fills)
            })
            .collect();

        Self { rows, cols }
    }

    pub fn row(&self, r: u16) -> &Rule {
        &self.rows[r as usize]
    }
    pub fn col(&self, c: u16) -> &Rule {
        &self.cols[c as usize]
    }
    pub fn line(&self, line: Line) -> Option<&Rule> {
        match line {
            Line::Row(r) if r < self.rows.len() => Some(&self.rows[r]),
            Line::Col(c) if c < self.cols.len() => Some(&self.cols[c]),
            _ => None,
        }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn from_serde(rules: SerdeRules, row_count: usize, col_count: usize) -> Self {
        let rows: Vec<_> = rules
            .rows
            .into_iter()
            .map(|runs| Rule::new(runs, row_count))
            .collect();
        let cols: Vec<_> = rules
            .cols
            .into_iter()
            .map(|runs| Rule::new(runs, col_count))
            .collect();

        Self { rows, cols }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn to_serde(&self) -> SerdeRules {
        let rows: Vec<_> = self.rows.iter().map(|rule| rule.runs.clone()).collect();
        let cols: Vec<_> = self.cols.iter().map(|rule| rule.runs.clone()).collect();

        SerdeRules { rows, cols }
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct SerdeRules {
    rows: Vec<Vec<Run>>,
    cols: Vec<Vec<Run>>,
}
