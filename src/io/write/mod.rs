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

mod checksums;
mod extras;
mod grids;
mod header;
mod strings;

pub(crate) use grids::*;
pub(crate) use header::*;

use std::io::{self, Write};

use crate::{Puzzle, io::Strings};

#[derive(Debug, Default)]
pub struct PuzWriter;

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
}

impl<W: Write> PuzWrite for W {}

impl PuzWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write<W: PuzWrite>(&self, writer: &mut W, puzzle: &Puzzle) -> io::Result<()> {
        // Write all (unordered) bytes into memory
        let mut header = self.write_header(puzzle)?;
        let grids = self.write_grids(puzzle);
        let strings = Strings::from_puzzle(puzzle)?;
        let extras = self.write_extras(puzzle)?;

        self.write_checksums(&mut header, &grids, &strings)?;

        // Write all (ordered) bytes into the writer
        writer.write_all(&header.cursor.into_inner())?;
        writer.write_all(&grids.solution)?;
        writer.write_all(&grids.state)?;
        strings.write_with(writer)?;
        writer.write_all(&extras)?;

        Ok(())
    }
}
