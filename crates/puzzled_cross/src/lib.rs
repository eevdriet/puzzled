//! The [`puzzled_cross`](crate) library provides functionality for reading, writing and solving [crossword](https://en.wikipedia.org/wiki/Crossword) puzzles.
//! A [`Crossword`] is either constructed directly from its [squares](Squares) and [clues](Clues) or using one of the readers from the [`io`] module.
//! ```
//! use puzzled_crossword::crossword;
//!
//! let puzzle = crossword! (
//!     [. . C A F]
//!     [. D E L I]
//!     [S E D E R]
//!     [I M A X .]
//!     [N O R . .]
//!
//!     - A: "Half-___ (coffee order)"
//!     - A: "Sandwich shop"
//!     - A: "Passover feast"
//!     - A: "Gigantic movie format"
//!     - A: "Neither's partner"
//!     - D: "Tree type"
//!     - D: "Trebek of Jeopardy!"
//!     - D: "Another tree type"
//!     - D: "Tape given to record labels"
//!     - D: "Sloth, e.g."
//! );
//!
//! assert_eq!(puzzle.rows(), 5);
//! assert_eq!(puzzle.cols(), 5);
//! ```
//!
//! # Features and dependencies
//! The library currently has no dependencies, expect for the following features:
//! - **serde** - Enables support for serializing and deserializing puzzles with [`serde`][serde]
//! - [`thiserror`][thiserror] to simplify the definition of the [`Error`] type and subtypes
//!
//! [puzzled]: crate
//! [PUZ spec]: https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5
//! [PUZ google spec]: https://code.google.com/archive/p/puz/wikis/FileFormat.wiki
//! [serde]: https://docs.rs/serde
//! [thiserror]: https://docs.rs/serde

pub mod io;
pub mod puzzle;

#[doc(hidden)]
pub use puzzled_core::*;

#[doc(hidden)]
pub use {io::*, puzzle::*};
