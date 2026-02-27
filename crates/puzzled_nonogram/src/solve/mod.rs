mod constraints;
mod state;
mod validate;

pub use constraints::*;
use puzzled_core::{Grid, Solver};
pub use state::*;
pub use validate::*;

use crate::{Fill, Nonogram};

#[derive(Debug, Default)]
pub struct NonogramSolver {}

impl Solver<Nonogram, NonogramState> for NonogramSolver {
    type Error = String;

    fn solve(
        &mut self,
        _puzzle: &Nonogram,
        _state: &mut NonogramState,
    ) -> Result<Grid<Fill>, String> {
        Ok(Grid::new(0, 0).expect("Yeet"))
    }
}
