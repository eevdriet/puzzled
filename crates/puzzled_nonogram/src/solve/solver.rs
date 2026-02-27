use crate::{Fill, Nonogram, NonogramState};
use puzzled_core::{Grid, GridError, Solver, SolverError};

#[derive(Debug, Default)]
pub struct NonogramSolver {}

impl Solver<Nonogram, NonogramState> for NonogramSolver {
    type Error = SolverError<GridError>;

    fn solve(
        &mut self,
        _puzzle: &Nonogram,
        _state: &mut NonogramState,
    ) -> Result<Grid<Fill>, Self::Error> {
        Ok(Grid::new(0, 0).expect("Yeet"))
    }
}
