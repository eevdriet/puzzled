mod constraints;
mod state;
mod validate;

pub use constraints::*;
use puzzled_core::{Grid, Solve, Solver};
pub use state::*;
pub use validate::*;

use crate::{Fill, Nonogram};

#[derive(Debug, Default)]
pub struct NonogramSolver {}

impl Solver<Nonogram> for NonogramSolver {
    fn solve<S>(&mut self, _puzzle: &Nonogram, _state: &mut S) -> Grid<Fill>
    where
        S: Solve<Nonogram>,
    {
        Grid::new(0, 0).expect("Temporary")
    }
}

impl NonogramSolver {
    pub fn new() -> Self {
        Self::default()
    }
}
