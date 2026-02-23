#![cfg_attr(docsrs, feature(doc_cfg))]

//! The [`nonogram`](crate) library provides functionality for reading, writing and solving [nonograms](https://en.wikipedia.org/wiki/Crossword).
//! A [`Nonogram`] is either constructed directly from its [fills](Fills) and [colors](Colors) or using one of the readers from the [`io`] module.
//! # Features
#![doc = document_features::document_features!()]

pub mod io;
pub mod puzzle;
pub mod solver;

#[doc(hidden)]
pub use puzzled_core::*;

#[doc(hidden)]
pub use {io::*, puzzle::*, solver::*};

#[cfg(feature = "macros")]
mod macros;
