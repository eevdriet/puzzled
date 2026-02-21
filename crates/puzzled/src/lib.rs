#![cfg_attr(docsrs, feature(doc_cfg))]

pub use puzzled_core as core;

#[cfg(feature = "crossword")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossword")))]
pub use puzzled_crossword as crossword;

#[cfg(feature = "nonogram")]
#[cfg_attr(docsrs, doc(cfg(feature = "nonogram")))]
pub use puzzled_nonogram as nonogram;

#[cfg(feature = "puz")]
#[cfg_attr(docsrs, doc(cfg(feature = "puz")))]
pub use puzzled_puz as puz;
