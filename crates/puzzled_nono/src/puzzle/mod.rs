mod error;
mod fill;
mod find;
mod rule;
mod run;
mod style;

use bitvec::vec::BitVec;
use puzzled_core::Line;
use std::collections::HashMap;

pub use error::*;
pub use fill::*;
pub use find::*;
pub use rule::*;
pub use run::*;
pub use style::*;

#[derive(Debug, Default)]
pub struct Nonogram {
    fills: Fills,
    rules: Rules,
    colors: Vec<Color>,
}

impl Nonogram {
    pub fn new(fills: Fills, rules: Rules, colors: Vec<Color>) -> Self {
        Self {
            fills,
            rules,
            colors,
        }
    }

    pub fn fills(&self) -> &Fills {
        &self.fills
    }

    pub fn fills_mut(&mut self) -> &mut Fills {
        &mut self.fills
    }

    pub fn rules(&self) -> &Rules {
        &self.rules
    }

    pub fn rules_mut(&mut self) -> &mut Rules {
        &mut self.rules
    }

    pub fn colors(&self) -> &Vec<Color> {
        &self.colors
    }

    /// Number of columns in the grid
    pub fn cols(&self) -> usize {
        self.fills.cols()
    }

    /// Number of rows in the grid
    pub fn rows(&self) -> usize {
        self.fills.rows()
    }
}

pub type LineMap<T> = HashMap<Line, T>;

pub type LineMask = BitVec;
