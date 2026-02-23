#![cfg_attr(docsrs, feature(doc_cfg))]

mod io;
mod puzzle;

pub use io::*;
pub use puzzle::*;

#[cfg(feature = "macros")]
mod macros;
