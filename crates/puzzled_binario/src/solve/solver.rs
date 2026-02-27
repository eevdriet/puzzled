use puzzled_core::{Direction, Grid, GridError, Solve, Solver, SolverError};

use crate::{Binario, BinarioState, Bit, Bits};

#[derive(Debug, Default)]
pub struct BinarioSolver {}

impl Solver<Binario, BinarioState> for BinarioSolver {
    type Error = SolverError<GridError>;

    fn solve(
        &mut self,
        _puzzle: &Binario,
        state: &mut BinarioState,
    ) -> Result<Grid<Bit>, Self::Error> {
        self.init(state);
        self.propagate(state);

        state.try_finalize().map_err(SolverError::CannotFinalize)
    }
}

impl BinarioSolver {
    fn init(&self, state: &mut BinarioState) {
        let solutions = &state.state.solutions;

        for (pos, bit) in solutions
            .positions()
            .filter_map(|pos| solutions.is_candidate(pos).map(|bit| (pos, bit)))
        {
            state.frontier.push_back((pos, bit));
        }
    }

    fn propagate(&self, state: &mut BinarioState) {
        while let Some((pos, bit)) = state.frontier.pop_front() {
            state.solve(&pos, bit);

            for dir in Direction::ALL {
                if let Some(adj) = pos + dir
                    && let Some(bit) = state.state.solutions.is_candidate(adj)
                {
                    state.frontier.push_back((adj, bit));
                }
            }
        }
    }
}
