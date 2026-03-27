mod solver;
mod state;

pub use solver::*;
pub use state::*;

use crate::Puzzle;

pub trait Solve<P: Puzzle> {
    fn solution(&self, pos: &P::Position) -> Option<&P::Value>;
    fn entry(&self, pos: &P::Position) -> Option<&P::Value>;

    fn solve(&mut self, pos: &P::Position, solution: P::Value) -> bool;
    fn enter(&mut self, pos: &P::Position, entry: P::Value) -> bool;
    fn clear(&mut self, pos: &P::Position) -> bool;
    fn reveal(&mut self, pos: &P::Position) -> bool;
    fn check(&mut self, pos: &P::Position) -> Option<bool>;

    fn guess(&mut self, pos: &P::Position, guess: P::Value) -> bool {
        self.enter(pos, guess)
    }
}
