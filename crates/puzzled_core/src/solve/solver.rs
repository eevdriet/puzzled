use crate::{Puzzle, Solve};

pub trait Solver<S>
where
    S: Solve,
{
    type Puzzle: Puzzle;
    type Error;

    fn solve(
        &mut self,
        puzzle: &Self::Puzzle,
        state: &mut S,
    ) -> Result<<Self::Puzzle as Puzzle>::Solution, Self::Error>;

    fn try_finalize(&self, state: &S) -> Result<<Self::Puzzle as Puzzle>::Solution, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum SolverError<F> {
    #[error("Solver cannot make further progress")]
    Stuck,

    #[error("Solver cannot finalize the solution: {0}")]
    CannotFinalize(F),
}
