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
use crate::io::{FILE_MAGIC, Span};

pub struct PuzWriter {
    buf: Vec<u8>,
}

#[derive(Debug)]
pub(crate) struct Header<'a> {
    // Checksums
    pub file_checksum: u16,
    pub cib_checksum: u16,
    pub low_checksums: &'a [u8],
    pub high_checksums: &'a [u8],

    // Regions
    pub cib_span: Span,
    pub masks_span: Span,
}

impl PuzWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn write(&mut self, puzzle: &Puzzle) -> Vec<u8> {
        self.buf.clear();

        let header = self.write_header(puzzle);
        self.write_strings(puzzle);
        self.write_grids(puzzle);
        self.write_extras(puzzle);

        self.buf.clone()
    }

    pub(crate) fn write_header(&mut self, puzzle: &Puzzle) {
        // Leave 2 bytes for the file checksum
        self.buf.extend([0, 0]);

        // File magic
        self.buf.extend(FILE_MAGIC.as_bytes());
    }
    pub(crate) fn write_strings(&mut self, puzzle: &Puzzle) {}
    pub(crate) fn write_grids(&mut self, puzzle: &Puzzle) {}
    pub(crate) fn write_extras(&mut self, puzzle: &Puzzle) {}

    pub(crate) fn write_span<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> (T, Span) {
        let start = self.buf.len() - 1;

        let value = f(self);

        let end = self.buf.len() - 1;
        let span = start..end;

        (value, span)
    }
}
