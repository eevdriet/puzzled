use crate::{Puzzle, Solve};

pub trait Solver<P, S>
where
    P: Puzzle,
    S: Solve<P>,
{
    type Error;

    fn solve(&mut self, puzzle: &P, state: &mut S) -> Result<P::Solution, Self::Error>;

    fn try_finalize(&self, state: &S) -> Result<P::Solution, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum SolverError<F> {
    #[error("Solver cannot make further progress")]
    Stuck,

    #[error("Solver cannot finalize the solution: {0}")]
    CannotFinalize(F),
}
