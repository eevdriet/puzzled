#![cfg_attr(docsrs, feature(doc_cfg))]

//! Read, write and solve [{{ puzzle }}](https://en.wikipedia.org/wiki/) puzzles.
//!
//! A [`{{ puzzle | pascal_case }}`] is either constructed directly or using one of the readers from the [`io`](puzzled_io) crate.
//!
//! ```
//! use puzzled::{{ puzzle }}::{{ puzzle }};
//!
//! let puzzle = {{ puzzle }}! (
//! );
//! ```
//!
//! # Features
#![doc = document_features::document_features!()]

mod io;
mod puzzle;

#[doc(hidden)]
pub use puzzled_core::*;

#[doc(inline)]
pub use puzzle::*;

#[cfg(feature = "macros")]
mod macros;

#[cfg(feature = "macros")]
#[doc(hidden)]
pub use macros::*;
