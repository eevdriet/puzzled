mod solver;
mod state;

pub use solver::*;
pub use state::*;

use crate::Puzzle;

pub trait Solve<P>
where
    P: Puzzle,
{
    type Value: Clone + Eq;
    type Position;

    fn solve(&mut self, pos: &Self::Position, solution: Self::Value) -> bool;
    fn enter(&mut self, pos: &Self::Position, entry: Self::Value) -> bool;
    fn reveal(&mut self, pos: &Self::Position) -> bool;
    fn check(&mut self, pos: &Self::Position) -> Option<bool>;

    fn reveal_all(&mut self);
    fn check_all(&mut self);

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

    fn try_finalize(&self) -> Result<P::Solution, Box<dyn std::error::Error>>;
}

#[macro_export]
macro_rules! forward_solve {
    ($wrapper:ty, $inner:ty, $puzzle:ty) => {
        impl Solve<$puzzle> for $wrapper {
            type Value = <$inner as Solve<$puzzle>>::Value;
            type Position = <$inner as Solve<$puzzle>>::Position;
            fn solve(&mut self, pos: &Self::Position, solution: Self::Value) -> bool {
                <_ as Solve<$puzzle>>::solve(&mut self.0, pos, solution)
            }

            fn enter(&mut self, pos: &Self::Position, entry: Self::Value) -> bool {
                <_ as Solve<$puzzle>>::enter(&mut self.0, pos, entry)
            }

            fn reveal(&mut self, pos: &Self::Position) -> bool {
                <_ as Solve<$puzzle>>::reveal(&mut self.0, pos)
            }

            fn check(&mut self, pos: &Self::Position) -> Option<bool> {
                <_ as Solve<$puzzle>>::check(&mut self.0, pos)
            }

            fn reveal_all(&mut self) {
                <_ as Solve<$puzzle>>::reveal_all(&mut self.0)
            }

            fn check_all(&mut self) {
                <_ as Solve<$puzzle>>::check_all(&mut self.0)
            }

            fn enter_checked(&mut self, pos: &Self::Position, entry: Self::Value) -> Option<bool> {
                <_ as Solve<$puzzle>>::enter_checked(&mut self.0, pos, entry)
            }

            fn guess(&mut self, pos: &Self::Position, guess: Self::Value) -> bool {
                <_ as Solve<$puzzle>>::guess(&mut self.0, pos, guess)
            }

            fn guess_checked(&mut self, pos: &Self::Position, guess: Self::Value) -> Option<bool> {
                <_ as Solve<$puzzle>>::guess_checked(&mut self.0, pos, guess)
            }

            fn try_finalize(
                &self,
            ) -> Result<<$puzzle as $crate::Puzzle>::Solution, Box<dyn std::error::Error>> {
                <_ as Solve<$puzzle>>::try_finalize(&self.0)
            }
        }
    };
}
