mod cell;
mod constraints;
mod geom;
mod metadata;
mod square;
mod style;
mod word;

use std::fmt::Debug;

pub use cell::*;
pub use constraints::*;
pub use geom::*;
pub use metadata::*;
pub use square::*;
pub use style::*;
pub use word::*;

use crate::{Solve, Solver};

pub trait Puzzle: Sized {
    const NAME: &'static str;

    type Solution;
    type Position: Debug;
    type Value: Debug + Clone + Eq + Word;

    fn title(meta: &Metadata) -> String {
        let title = meta.title().unwrap_or(Self::NAME);

        match meta.author() {
            Some(author) => format!("{title} (by {author})"),
            None => title.to_string(),
        }
    }

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
