#![cfg_attr(docsrs, feature(doc_cfg))]

mod puzzle;

#[doc(inline)]
pub use puzzle::*;

#[cfg(feature = "macros")]
mod macros;
