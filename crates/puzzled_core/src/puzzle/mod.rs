mod cell;
mod geom;
mod metadata;
mod square;
mod style;

pub use cell::*;
pub use geom::*;
pub use metadata::*;
pub use square::*;
pub use style::*;

use crate::{Solve, Solver};

pub trait Puzzle: Sized {
    type Solution;

    fn solve_with<'a, S, T>(&'a self, solver: &mut S) -> Result<Self::Solution, S::Error>
    where
        S: Solver<Self, T>,
        T: Solve<Self> + From<&'a Self>,
    {
        let mut state = T::from(self);
        solver.solve(self, &mut state)
    }

    fn solve_with_state<S, T>(
        &self,
        solver: &mut S,
        state: &mut T,
    ) -> Result<Self::Solution, S::Error>
    where
        S: Solver<Self, T>,
        T: Solve<Self>,
    {
        solver.solve(self, state)
    }
}
