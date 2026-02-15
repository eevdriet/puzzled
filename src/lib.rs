//! The [puzzled] library provides functionality for parsing, constructing and solving crossword puzzles.
//! [Puzzles](Puzzle) are either constructed directly from its [squares](Square) and [clues](Clue) or by [parsing](Parser) from byte data.
//! For the latter, the data should follow the [Across Lite *.puz format][PUZ google spec], which has been the de-facto standard for sharing crossword puzzles online.
//!
//! Below is a minimal example of the functionalities of [`Puzzle`], which is the primary type that the library provides:
//! ```
//! use puzzled::puzzle;
//!
//! let puzzle = puzzle! (
//!     [. . C A F]
//!     [. D E L I]
//!     [S E D E R]
//!     [I M A X .]
//!     [N O R . .]
//!     ---
//!     A: "Half-___ (coffee order)",
//!     A: "Sandwich shop",
//!     A: "Passover feast",
//!     A: "Gigantic movie format",
//!     A: "Neither's partner",
//!     D: "Tree type",
//!     D: "Trebek of Jeopardy!",
//!     D: "Another tree type",
//!     D: "Tape given to record labels",
//!     D: "Sloth, e.g.",
//! );
//!
//! assert_eq!(puzzle.rows(), 5);
//! assert_eq!(puzzle.cols(), 5);
//! ```
//!
//! # Usage
//! The primary type in this crate is [`Puzzle`], which represents a crossword puzzle.
//! It can be constructed in the following ways
//! 1.  [`puzzle!`] allows for defining an static puzzle, by directly specifying a square grid and optionally some clues.
//!     The clues are then placed based on which
//! 2.  [`Puzzle::from_squares`] is the dynamic counterpart to [`puzzle!`].
//!     In similar fashion, it takes a [square grid](`Grid`) that initializes the puzzle's [squares](Square).
//!     After constructing the puzzle, [`Puzzle::insert_clues`] can be used to add the puzzle entries to provide clues to the squares.
//! 3.  [`Parser::parse`] and friends let you parse a puzzle from series of bytes that represent a [*.puz file][PUZ spec].
//!     By default the parser ignores [warnings](crate::parse::Warning) that arise from recoverable input [errors](crate::parse::Error).
//!     By default the parser ignores [warnings](crate::parse::Warning) that come from [invalid checksums](crate::parse#validating-checksums) or [corrupted extra sections](crate::parse#extra-sections).
//!
//! The following all construct the same puzzle:
//! ```no_run
//! use puzzled::{clue, Grid, puzzle, Puzzle, square};
//! use puzzled::parse::{PuzParser, TxtParser};
//!
//! // 1. Static
//! let puzzle1 = puzzle! {
//!     [A B]
//!     [C .]
//!     ---
//!     A: "The first two letters of the alphabet",
//!     D: "Keep it short, but cool"
//! };
//!
//! // 2. Dynamic
//! let squares =
//!     Grid::new(vec![square!('A'), square!('B'), square!('C'), square!()], 2)
//!     .expect("Grid size evenly divides columns");
//!
//! let clues = vec![
//!     clue!(A: "The first two letters of the alphabet"),
//!     clue!(D: "Keep it short, but cool")
//! ];
//!
//! let mut puzzle2 = Puzzle::from_squares(squares);
//! puzzle2.insert_clues(clues);
//!
//! // 3a. Parsed from a binary .puz file
//! let parser = PuzParser::new(true);
//! let bytes = include_bytes!("../puzzles/ok/alphabet.puz");
//! let puzzle3 = parser.parse(bytes)?;
//!
//! // 3b. Parsed from a .txt file
//! let parser = TxtParser::new();
//! let text = include_str!("../puzzles/ok/alphabet.txt");
//! let puzzle4 = parser.parse(text)?;
//!
//! assert_eq!(puzzle1, puzzle2);
//! assert_eq!(puzzle2, puzzle3);
//! assert_eq!(puzzle3, puzzle4);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Dependencies
//! The library currently has the following dependencies
//!     - [thiserror] to simplify the definition of the [`Error`](crate::parse::Error) type and subtypes
//!     - [bitflags] to define the different [`styles`](CellStyle) that can be set for a [`square`](Cell)
//!
//! # Features
//! Currently, [puzzled] defines the following flags
//!     - **serde** - Enables support for serializing puzzles
//!
//! [puzzled]: crate
//! [PUZ spec]: https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5
//! [PUZ google spec]: https://code.google.com/archive/p/puz/wikis/FileFormat.wiki

pub mod parse;
pub use parse::PuzParser;

pub mod write;
pub use write::Writer;

mod puzzle;
pub use puzzle::*;

pub(crate) const SECTION_SEPARATOR: &str = "---";
