mod solver;
mod state;

pub use solver::*;
pub use state::*;

use crate::Puzzle;

pub trait Solve<P: Puzzle> {
    fn solve(&mut self, pos: &P::Position, solution: P::Value) -> bool;
    fn enter(&mut self, pos: &P::Position, entry: P::Value) -> bool;
    fn clear(&mut self, pos: &P::Position) -> bool;
    fn reveal(&mut self, pos: &P::Position) -> bool;
    fn check(&mut self, pos: &P::Position) -> Option<bool>;

    fn reveal_all(&mut self);
    fn check_all(&mut self);
    fn clear_all(&mut self);

    fn enter_checked(&mut self, pos: &P::Position, entry: P::Value) -> Option<bool> {
        self.enter(pos, entry);
        self.check(pos)
    }

    fn guess(&mut self, pos: &P::Position, guess: P::Value) -> bool {
        self.enter(pos, guess)
    }

    fn guess_checked(&mut self, pos: &P::Position, guess: P::Value) -> Option<bool> {
        self.guess(pos, guess);
        self.check(pos)
    }
}
