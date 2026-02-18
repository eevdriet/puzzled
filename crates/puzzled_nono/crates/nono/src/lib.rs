#![allow(dead_code)]

mod error;
mod fill;
mod geom;
mod puzzle;
mod rules;
mod run;
mod solver;
mod style;

pub use fill::*;
pub use geom::*;
pub use puzzle::*;
pub use rules::*;
pub use run::*;
pub use style::*;

pub use error::*;
pub use solver::*;

pub struct Nonogram {
    pub puzzle: Puzzle,
    pub rules: Rules,
    pub colors: Vec<Color>,
}
