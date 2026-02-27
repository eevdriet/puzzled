#![cfg_attr(docsrs, feature(doc_cfg))]

//! Read, write and solve [binario](https://en.wikipedia.org/wiki/) puzzles.
//!
//! A [`Binario`] is either constructed directly or using one of the readers from the [`io`](puzzled_io) crate.
//!
//! ```
//! use puzzled::binario::binario;
//!
//! let puzzle = binario! (
//! );
//! ```
//!
//! # Features
#![doc = document_features::document_features!()]

mod io;
mod puzzle;
mod solve;

#[doc(hidden)]
pub use puzzled_core::*;

#[doc(inline)]
pub use {puzzle::*, solve::*};

#[cfg(feature = "macros")]
mod macros;
