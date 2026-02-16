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

mod header;

pub(crate) use header::*;

use std::io::{self, Cursor, Seek, SeekFrom, Write};

use crate::io::{FILE_MAGIC, MISSING_ENTRY_CELL, NON_PLAYABLE_CELL, Span, find_cib_checksum};
use crate::{Puzzle, Square};

#[derive(Debug, Default)]
pub struct PuzWriter;

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

pub trait PuzWrite: Write {
    fn pad(&mut self, pad: usize) -> io::Result<()> {
        self.write_all(&vec![0; pad])
    }

    fn write_u8(&mut self, val: u8) -> io::Result<()> {
        self.write_all(&[val])
    }

    fn write_u16(&mut self, val: u16) -> io::Result<()> {
        self.write_all(&val.to_le_bytes())
    }

    fn write_str0(&mut self, val: &str) -> io::Result<()> {
        self.write_all(val.as_bytes())?;
        self.write_u8(b'\0')
    }

    fn write_opt_str0(&mut self, str: Option<&str>, pad: usize) -> io::Result<()> {
        match str {
            Some(str) => self.write_str0(str),
            None => {
                self.pad(pad)?;
                self.write_u8(b'\0')
            }
        }
    }

    // fn write_span(&mut self, f: impl FnOnce(&mut Self) -> io::Result<()>) -> io::Result<Span> {
    //     let start = self.stream_position()? as usize;
    //     f(self)?;
    //     let end = self.stream_position()? as usize;
    //
    //     Ok(start..end)
    // }
}

impl<W: Write> PuzWrite for W {}

impl PuzWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write<W: PuzWrite>(&self, writer: &mut W, puzzle: &Puzzle) -> io::Result<()> {
        let header = self.write_header(writer, puzzle)?;
        self.write_grids(writer, puzzle)?;
        self.write_strings(writer, puzzle)?;
        self.write_extras(writer, puzzle)?;

        Ok(())
    }

    pub(crate) fn write_grids<W: PuzWrite>(
        &self,
        writer: &mut W,
        puzzle: &Puzzle,
    ) -> io::Result<()> {
        for square in puzzle.iter() {
            let byte = match square {
                Square::Black => NON_PLAYABLE_CELL,
                Square::White(cell) => {
                    cell.solution();
                    4
                }
            };

            writer.write_u8(byte)?;
        }

        for square in puzzle.iter() {
            let byte = match square {
                Square::Black => NON_PLAYABLE_CELL,
                Square::White(cell) => match cell.entry() {
                    Some(v) => v.chars().next().unwrap_or(MISSING_ENTRY_CELL as char) as u8,
                    None => MISSING_ENTRY_CELL,
                },
            };

            writer.write_u8(byte)?;
        }

        Ok(())
    }

    pub(crate) fn write_strings<W: PuzWrite>(
        &self,
        writer: &mut W,
        puzzle: &Puzzle,
    ) -> io::Result<()> {
        writer.write_opt_str0(puzzle.title(), 0)?;
        writer.write_opt_str0(puzzle.author(), 0)?;
        writer.write_opt_str0(puzzle.copyright(), 0)?;

        for clue in puzzle.iter_clues() {
            writer.write_str0(&clue.text)?;
        }

        writer.write_opt_str0(puzzle.notes(), 0)?;
        Ok(())
    }

    pub(crate) fn write_extras<W: PuzWrite>(
        &self,
        writer: &mut W,
        puzzle: &Puzzle,
    ) -> io::Result<()> {
        Ok(())
    }
}
