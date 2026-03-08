mod solver;
mod state;

pub use solver::*;
pub use state::*;

use crate::Puzzle;

pub trait Solve {
    type Puzzle: Puzzle;
    type Value: Clone + Eq;
    type Position;
    type Error;

    fn solve(&mut self, pos: &Self::Position, solution: Self::Value) -> bool;
    fn enter(&mut self, pos: &Self::Position, entry: Self::Value) -> bool;
    fn clear(&mut self, pos: &Self::Position) -> bool;
    fn reveal(&mut self, pos: &Self::Position) -> bool;
    fn check(&mut self, pos: &Self::Position) -> Option<bool>;

    fn reveal_all(&mut self);
    fn check_all(&mut self);
    fn clear_all(&mut self);

    fn enter_checked(&mut self, pos: &Self::Position, entry: Self::Value) -> Option<bool> {
        self.enter(pos, entry);
        self.check(pos)
    }

    fn guess(&mut self, pos: &Self::Position, guess: Self::Value) -> bool {
        self.enter(pos, guess)
    }

    fn guess_checked(&mut self, pos: &Self::Position, guess: Self::Value) -> Option<bool> {
        self.guess(pos, guess);
        self.check(pos)
    }

    fn try_finalize(&self) -> Result<<Self::Puzzle as Puzzle>::Solution, Self::Error>;
}
