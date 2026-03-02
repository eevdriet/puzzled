mod cell;
mod error;
mod grid;
mod metadata;
mod square;
mod util;

pub use cell::*;
pub use error::*;
pub use grid::*;
pub use square::*;
pub use util::*;

use crate::chumsky::TxtPuzzle;

use chumsky::Parser;
use std::{fs, path::Path};

pub struct TxtReader {
    strict: bool,
}

impl TxtReader {
    pub fn new(strict: bool) -> Self {
        Self { strict }
    }

    pub fn read<P, S>(&self, input: &str) -> Result<(P, S)>
    where
        P: TxtPuzzle<S>,
    {
        P::read_text()
            .parse(input)
            .into_output()
            .ok_or(std::io::Error::other("Parsing failed").into())
    }

    pub fn read_from_path<R, P, S>(&self, path: R) -> Result<(P, S)>
    where
        R: AsRef<Path>,
        P: TxtPuzzle<S>,
    {
        let file_str = fs::read_to_string(path)?;
        println!("File: {file_str}");
        self.read(&file_str)
    }
}
