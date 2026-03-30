mod cell;
mod colors;
mod fill;
mod find;
mod rule;
mod run;

use std::{collections::HashMap, fmt};

use derive_more::{Index, IndexMut};
use puzzled_core::{Cell, Grid, Line, Metadata, Position, Puzzle};

pub use cell::*;
pub use colors::*;
pub use fill::*;
pub use find::*;
pub use rule::*;
pub use run::*;

#[derive(Debug, Index, IndexMut)]
pub struct Nonogram {
    #[index]
    #[index_mut]
    fills: Grid<Cell<Fill>>,

    rules: Rules,
    colors: Colors,

    meta: Metadata,
}

impl Puzzle for Nonogram {
    const NAME: &'static str = "Nonogram";

    type Solution = Grid<Fill>;
    type Position = Position;
    type Value = Fill;
}

impl fmt::Display for Nonogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.fills)?;

        if !self.colors.is_empty() {
            writeln!(f, "{}", self.colors)?;
        }

        writeln!(f, "{}", self.meta)?;

        Ok(())
    }
}

impl Nonogram {
    pub fn new(fills: Grid<Cell<Fill>>, colors: Colors, meta: Metadata) -> Self {
        Self {
            fills,
            rules: Rules::default(),
            colors,
            meta,
        }
    }

    pub fn with_rules(mut self, rules: Rules) -> Self {
        self.rules = rules;
        self
    }
    pub fn with_line_runs(mut self, line_runs: HashMap<Line, Vec<Run>>) -> Self {
        for (line, runs) in line_runs {
            let line_len = self.fills.line_len(line);
            let rule = Rule::new(runs, line_len);

            self.rules.insert(line, rule);
        }

        self
    }

    pub fn fills(&self) -> &Grid<Cell<Fill>> {
        &self.fills
    }

    pub fn fills_mut(&mut self) -> &mut Grid<Cell<Fill>> {
        &mut self.fills
    }

    pub fn rules(&self) -> &Rules {
        &self.rules
    }

    pub fn rules_mut(&mut self) -> &mut Rules {
        &mut self.rules
    }

    pub fn colors(&self) -> &Colors {
        &self.colors
    }

    pub fn colors_mut(&mut self) -> &mut Colors {
        &mut self.colors
    }

    /// Number of columns in the grid
    pub fn cols(&self) -> usize {
        self.fills.cols()
    }

    /// Number of rows in the grid
    pub fn rows(&self) -> usize {
        self.fills.rows()
    }

    pub fn meta(&self) -> &Metadata {
        &self.meta
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use puzzled_core::{Cell, Grid, Metadata};
    use serde::{Deserialize, Serialize, de};

    use crate::{Colors, Fill, Nonogram, Rules, SerdeRules};

    #[derive(Serialize, Deserialize)]
    struct SerdeNonogram {
        rows: usize,
        cols: usize,

        #[serde(skip_serializing_if = "Option::is_none")]
        fills: Option<Grid<Cell<Fill>>>,

        rules: SerdeRules,

        // #[serde(skip_serializing_if = "Colors::is_empty")]
        colors: Colors,

        #[serde(skip_serializing_if = "Option::is_none")]
        meta: Option<Metadata>,
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl Serialize for Nonogram {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let rules = self.rules.to_serde();
            let fills = self.fills.clone();
            let colors = self.colors.clone();
            let meta = self.meta.clone();

            SerdeNonogram {
                rules,
                fills: Some(fills),
                colors,
                meta: Some(meta),
                rows: self.rows(),
                cols: self.cols(),
            }
            .serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de> Deserialize<'de> for Nonogram {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let SerdeNonogram {
                rows,
                cols,
                fills: opt_fills,
                rules,
                colors,
                meta: opt_meta,
            } = SerdeNonogram::deserialize(deserializer)?;

            let fills = opt_fills
                .unwrap_or(Grid::new_from(rows, cols, Cell::default()).map_err(de::Error::custom)?);
            let meta = opt_meta.unwrap_or_default();

            let rules = Rules::from_serde(rules, rows, cols);
            let nonogram = Self {
                fills,
                rules,
                colors,
                meta,
            };

            Ok(nonogram)
        }
    }
}

#[cfg(test)]
mod tests {
    use puzzled_core::{CellStyle, Position};

    use crate::nonogram;

    #[test]
    fn nonogram() {
        let mut puzzle = nonogram!(
            [ 0 - 1 ]
            [ 0 a 0 ]
            [ - 1 b ]
            - b: "#23AF"
            - 0: "#FFF"
            - a: "#0000"

            version: "1.0"
            author: "Eertze"
            copyright: " Yeet"
            title : "My first puzzle"
        );
        puzzle[Position::new(0, 0)].style |= CellStyle::INCORRECT | CellStyle::REVEALED;

        print!("{puzzle}");
        print!("{puzzle:?}");

        // #[cfg(feature = "text")]
        // puzzle.save_text("yeet").unwrap();

        panic!("Auto fail")
    }
}
