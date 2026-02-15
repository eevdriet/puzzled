//! Defines all functionality for writing and serializing a [puzzle](Puzzle) into a valid [*.puz file][PUZ google spec]
//!
//! # Usage
//! The primary type for writing out [puzzles](Puzzle) is the [`Writer`], which writes to a [`Vec<T>`].
//! Depending on the desired output, this can be a
//! - [`Vec<u8>`] for writing to [*.puz files][PUZ google spec]
//! - [`Vec<u8>`] for writing to [*.puz files][PUZ google spec]
//!
//! [PUZ google spec]: https://code.google.com/archive/p/puz/wikis/FileFormat.wiki
//! [PUZ spec]: https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5

use crate::Puzzle;

pub struct Writer {
    buf: Vec<u8>,
}

impl Writer {
    pub fn write(&mut self, puzzle: Puzzle) {}
}
