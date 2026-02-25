#![cfg_attr(docsrs, feature(doc_cfg))]

// Puzzle
mod puzzle;

#[doc(inline)]
pub use puzzle::*;

// Solver
mod solver;

#[doc(inline)]
pub use solver::*;

// Macros
#[cfg(feature = "macros")]
mod macros;
