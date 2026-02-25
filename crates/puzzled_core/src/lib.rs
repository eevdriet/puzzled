#![cfg_attr(docsrs, feature(doc_cfg))]

// Puzzle
mod puzzle;

#[doc(inline)]
pub use puzzle::*;

// Solver
mod solve;

#[doc(inline)]
pub use solve::*;

// Macros
#[cfg(feature = "macros")]
mod macros;
