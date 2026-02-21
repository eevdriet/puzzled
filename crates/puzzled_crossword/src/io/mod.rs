//! Defines all functionality for reading and writing [puzzles](crate::Crossword)
//!
//! # Formats
//! The crate currently supports the following formats and streams for dealing with them:
//! | Format  | Reader | Writer |
//! |------------|--------|------|
//! | Binary | [`PuzReader`] | [`PuzWriter`] |
//! | Text | [`TxtReader`] | |
//!
//! ## Binary
//! This crate tries to following the [Across Lite format][PUZ google spec] as closely as possible to handle binary data.
//! Note that a [reformatted specification][PUZ spec] which may be easier to read from, as it provides the full specification in Markdown.
//! The format is used to create `*.puz` files, which are commonly shared online on platforms such as [Crosshare](https://crosshare.org/).
//!
//! ## Text
//! The **text** format allows for a more WYSIWYG definition of puzzles.
//! It ties in nicely with the [`crossword!`](crate::crossword!) macro, as its [DSL](https://doc.rust-lang.org/rust-by-example/macros/dsl.html) follows the text format exactly.
//!
//! For example, the following two ways to construct a puzzle are identical
//! ```
//! use puzzled::crossword::{crossword, io::TxtReader};
//! use std::{path::Path, fs::read_to_string};
//!
//! // 1. Macro definition
//! let puzzle1 = crossword! {
//!     [A B]
//!     [C .]
//!     - A: "The first two letters of the alphabet"
//!     - D: "Keep it short, but cool"
//!     version: "2.0"
//! };
//!
//! // 2. Text file
//! let path = Path::new("puzzles/ok/alphabet.txt");
//! let txt = read_to_string(path)?;
//!
//! let reader = TxtReader;
//! let puzzle2 = reader.read(&txt)?;
//!
//! assert_eq!(puzzle1, puzzle2);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#[cfg(feature = "puz")]
mod puz;

pub mod text;
pub use text::TxtReader;
pub(crate) use text::TxtState;
