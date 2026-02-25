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

pub trait Puzzle: Sized {
    type Solution;
    type State;

    fn initial_state(&self) -> Self::State;
}
