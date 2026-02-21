use derive_more::Debug;

use crate::{Nonogram, Rule};

#[derive(Debug, Default, Clone)]
pub struct Rules {
    pub rows: Vec<Rule>,
    pub cols: Vec<Rule>,
}

impl Rules {
    pub fn new(rows: Vec<Rule>, cols: Vec<Rule>) -> Self {
        Self { rows, cols }
    }

    pub fn from_puzzle(puzzle: &Nonogram) -> Self {
        let rows = puzzle
            .fills()
            .iter_rows()
            .map(|row| {
                let fills = row.copied();
                Rule::from_fills(fills)
            })
            .collect::<Vec<_>>();

        let cols = puzzle
            .fills()
            .iter_cols()
            .map(|col| {
                let fills = col.copied();
                Rule::from_fills(fills)
            })
            .collect::<Vec<_>>();

        Self { rows, cols }
    }

    pub fn row(&self, r: u16) -> &Rule {
        &self.rows[r as usize]
    }
    pub fn col(&self, c: u16) -> &Rule {
        &self.cols[c as usize]
    }
}
