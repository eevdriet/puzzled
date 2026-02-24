#![cfg_attr(docsrs, feature(doc_cfg))]

//! Read, write and solve [nonograms](https://en.wikipedia.org/wiki/Nonogram)
//!
//! A [`Nonogram`] is either constructed directly from its [fills](Fills) and [colors](Colors) or using one of the readers from the [`io`](puzzled_io) crate.
//! # Features
#![doc = document_features::document_features!()]

pub mod io;
pub mod puzzle;
pub mod solver;

#[doc(hidden)]
pub use puzzled_core::{cell as __core_cell, *};

#[doc(inline)]
pub use {io::*, puzzle::*, solver::*};

#[cfg(feature = "macros")]
mod macros;
