use crate::{Puzzle, Solve};

pub trait Solver<P>
where
    P: Puzzle,
{
    fn solve<S>(&mut self, puzzle: &P, state: &mut S) -> P::Solution
    where
        S: Solve<P>;
}
