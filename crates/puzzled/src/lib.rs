//! Puzzled
//!
//! # Features
#![doc = document_features::document_features!()]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc(hidden)]
pub use puzzled_core as core;

#[doc(hidden)]
#[cfg(feature = "crossword")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossword")))]
pub use puzzled_crossword as crossword;

#[doc(hidden)]
#[cfg(feature = "nonogram")]
#[cfg_attr(docsrs, doc(cfg(feature = "nonogram")))]
pub use puzzled_nonogram as nonogram;

#[doc(hidden)]
#[cfg(feature = "puz")]
#[cfg_attr(docsrs, doc(cfg(feature = "puz")))]
pub use puzzled_puz as puz;
