//! Defines all functionality for writing a [puzzle](Puz) as [*.puz data][PUZ google spec]
//!
//! # Usage
//! The primary type for writing out [puzzles](Crossword) is the [`Writer`], which writes to a [`Vec<T>`].
//! Depending on the desired output, this can be a
//! - [`Vec<u8>`] for writing to [*.puz files][PUZ google spec]
//! - [`Vec<u8>`] for writing to [*.puz files][PUZ google spec]
//!
//! [PUZ google spec]: https://code.google.com/archive/p/puz/wikis/FileFormat.wiki
//! [PUZ spec]: https://gist.github.com/sliminality/dab21fa834eae0a70193c7cd69c356d5

mod error;
mod size;
mod util;

pub use error::*;
pub use size::*;
pub use util::*;

use puzzled_core::Metadata;

use std::io::{self, Write};

use crate::{
    Context,
    puz::{BinaryPuzzle, ByteStr, Grids, Header, Strings, write},
};

#[derive(Debug, Default)]
pub struct PuzWriter;

/// Extension trait for [`Write`] to make writing [puzzles](Crossword) to a [binary format](https://code.google.com/archive/p/puz/wikis/FileFormat.wiki) easier
///
/// Includes convenience methods for writing a [`u8`], [`u16`], [`str`] and [`Option<&str>`]
pub trait PuzWrite: Write {
    /// Pad the writer with `pad` 0-bytes
    fn pad(&mut self, pad: usize) -> io::Result<()> {
        self.write_all(&vec![0; pad])
    }

    /// Write a [`u8`]
    fn write_u8(&mut self, val: u8) -> io::Result<()> {
        self.write_all(&[val])
    }

    /// Write a [`u16`]
    fn write_u16(&mut self, val: u16) -> io::Result<()> {
        self.write_all(&val.to_le_bytes())
    }

    /// Write a [`str`] as a null-terminated string
    ///
    /// # Assumptions
    /// The argument does not already include a terminated `\0` byte
    fn write_byte_str(&mut self, str: &ByteStr) -> io::Result<()> {
        self.write_all(str.bytes(false))
    }
}

impl<W: Write> PuzWrite for W {}

impl PuzWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write<W, P, S>(&self, writer: &mut W, puzzle: &P, state: &S) -> Result<()>
    where
        W: PuzWrite,
        P: BinaryPuzzle<S>,
    {
        // Verify that the puzzle is sized correctly
        let width = puzzle.width();
        let height = puzzle.height();

        check_puz_size("Puzzle width", width, u8::MAX as usize)?;
        check_puz_size("Puzzle height", height, u8::MAX as usize)?;

        let clues = puzzle.clues();
        check_puz_size("Clue count", clues.len(), u16::MAX as usize)?;

        // Construct the individual sections from the puzzle
        let meta = puzzle.metadata();

        let mut header = self.build_header(puzzle, clues.len() as u16, &meta);
        let strings = self.build_strings(clues, &meta);
        let grids = self.build_grids(puzzle, state)?;
        let extras = puzzle.extras(state)?;

        self.write_checksums(&mut header, &grids, &strings);

        // Write all sections into the writer
        header.write_with(writer)?;
        grids.write_with(writer)?;
        strings.write_with(writer)?;
        extras.write_with(writer)?;

        Ok(())
    }

    pub fn build_header<P, S>(
        &self,
        puzzle: &P,
        clue_count: u16,
        metadata: &Option<&Metadata>,
    ) -> Header
    where
        P: BinaryPuzzle<S>,
    {
        let mut header = Header {
            width: puzzle.width() as u8,
            height: puzzle.height() as u8,
            clue_count,
            ..Default::default()
        };

        if let Some(Some(version)) = metadata.map(|m| m.version()) {
            header.version = version.as_bytes();
        }

        header.write_cib();
        header
    }

    pub fn build_grids<P, S>(&self, puzzle: &P, state: &S) -> write::Result<Grids>
    where
        P: BinaryPuzzle<S>,
    {
        let (solution, state) = puzzle.grids(state)?;
        let grids = Grids {
            solution,
            state,
            width: puzzle.width() as u8,
            height: puzzle.height() as u8,
        };

        grids.validate().context("Building grids")?;
        Ok(grids)
    }

    pub fn build_strings(&self, clues: Vec<ByteStr>, metadata: &Option<&Metadata>) -> Strings {
        let meta = match metadata {
            Some(m) => m,
            None => &Metadata::default(),
        };

        let mut strings = Strings::from_metadata(meta);
        strings.clues = clues;

        strings
    }
}
