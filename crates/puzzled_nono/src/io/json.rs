use std::path::Path;

use puzzled_core::Grid;
use serde::Deserialize;

use crate::{Fill, Fills, Nonogram, Rule, Rules, Run, io};

pub struct JsonLoader;

impl io::PuzzleLoader for JsonLoader {
    fn load_nonogram(path: &Path) -> io::Result<Nonogram> {
        let text = std::fs::read_to_string(path)?;
        let raw: JsonNonogram = serde_json::from_str(&text)?;
        let nonogram = raw.try_into()?;

        Ok(nonogram)
    }
}

#[derive(Debug, Deserialize)]
pub struct JsonNonogram {
    pub colors: Vec<(u8, u8, u8)>,
    pub rows: Vec<Vec<RawRun>>,
    pub cols: Vec<Vec<RawRun>>,

    #[serde(default)]
    pub puzzle: Vec<Vec<usize>>,
}

#[derive(Debug, Deserialize)]
pub struct RawRun {
    pub fill: usize,
    pub count: usize,
}

impl TryFrom<JsonNonogram> for Nonogram {
    type Error = io::Error;

    fn try_from(data: JsonNonogram) -> core::result::Result<Self, Self::Error> {
        let fills: Vec<_> = data
            .puzzle
            .iter()
            .flatten()
            .map(|id| Fill::Color(*id))
            .collect();

        let rows = data.rows.len();
        let cols = data.cols.len();

        let row_rules: Vec<_> = data
            .rows
            .into_iter()
            .map(|raw| {
                let runs: Vec<_> = raw
                    .iter()
                    .map(|run| Run::new(Fill::Color(run.fill), run.count))
                    .collect();

                Rule::new(runs, cols)
            })
            .collect();

        let col_rules: Vec<_> = data
            .cols
            .into_iter()
            .map(|raw| {
                let runs: Vec<_> = raw
                    .iter()
                    .map(|run| Run::new(Fill::Color(run.fill), run.count))
                    .collect();

                Rule::new(runs, rows)
            })
            .collect();

        let rules = Rules::new(row_rules, col_rules);
        let grid = if data.puzzle.is_empty() {
            Grid::new(rows, cols).expect("Non-overflowing size")
        } else {
            Grid::from_vec(fills, cols).expect("Correct size")
        };

        Ok(Nonogram::new(Fills::new(grid), rules, data.colors))
    }
}
