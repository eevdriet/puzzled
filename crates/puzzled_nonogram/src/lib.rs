#![cfg_attr(docsrs, feature(doc_cfg))]

//! Read, write and solve [nonograms](https://en.wikipedia.org/wiki/Nonogram)
//!
//! A [`Nonogram`] is either constructed directly from its [fills](Fills) and [colors](Colors) or using one of the readers from the [`io`](puzzled_io) crate.
//! # Features
#![doc = document_features::document_features!()]

pub mod io;
pub mod puzzle;
pub mod solve;

#[doc(inline)]
pub use {io::*, puzzle::*, solve::*};

#[doc(hidden)]
pub use puzzled_core::{Solve, Solver, cell as __core_cell, *};

#[cfg(feature = "macros")]
mod macros;
