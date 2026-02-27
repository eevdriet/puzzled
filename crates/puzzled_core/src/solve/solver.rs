use crate::{Puzzle, Solve};

pub trait Solver<P, S>
where
    P: Puzzle,
    S: Solve<P>,
{
    type Error;

    fn solve(&mut self, puzzle: &P, state: &mut S) -> Result<P::Solution, Self::Error>;
}
