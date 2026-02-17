//! The [puzzled] library provides functionality for reading, writing and solving crossword puzzles.
//! A [`Puzzle`] is either constructed directly from its [squares](Square) and [clues](Clue) or using one of the readers from the [`io`] module.
//! See
//! - [`puzzle`](crate::puzzle) for constructing and interacting with a [`Puzzle`] and its related types
//! - [`io`] for reading and writing puzzles into the various formats that are supported
//! ```
//! use puzzled::puzzle;
//!
//! let puzzle = puzzle! (
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
//! - **serde** - Enables support for serializing and deserializing puzzles with [serde]
//! - [thiserror] to simplify the definition of the [`Error`](crate::io::Error) type and subtypes
//!
//! [puzzled]: crate
//! [PUZ spec]: https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5
//! [PUZ google spec]: https://code.google.com/archive/p/puz/wikis/FileFormat.wiki

pub mod io;
pub mod puzzle;

pub use {io::*, puzzle::*};
