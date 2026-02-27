mod state;

pub use state::*;

use puzzled_core::{Grid, Solve, Solver};

use crate::{Binario, Bit};

#[derive(Debug, Default)]
pub struct BinarioSolver {}

impl Solver<Binario> for BinarioSolver {
    fn solve<S>(&mut self, _puzzle: &Binario, _state: &mut S) -> Grid<Bit>
    where
        S: Solve<Binario>,
    {
        Grid::new(0, 0).expect("Temporary")
    }
}

impl BinarioSolver {
    pub fn new() -> Self {
        Self::default()
    }
}
