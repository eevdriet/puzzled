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

pub use error::*;
pub use size::*;

use std::io::{self, Write};

use crate::puz::{BinaryPuzzle, ByteStr};

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
        self.write_all(str.bytes(true))
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
        // Construct the individual sections from the puzzle
        let mut header = puzzle.write_header(state)?;
        let grids = puzzle.write_grids(state)?;
        let strings = puzzle.write_strings(state)?;
        let extras = puzzle.write_extras(state)?;

        self.write_checksums(&mut header, &grids, &strings);

        // Write all sections into the writer
        header.write_with(writer)?;
        grids.write_with(writer)?;
        strings.write_with(writer)?;
        extras.write_with(writer)?;

        Ok(())
    }
}
