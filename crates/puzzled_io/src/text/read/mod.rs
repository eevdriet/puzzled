mod cell;
mod error;
mod grid;
mod metadata;
mod square;
mod util;

use std::{fs, path::Path};

pub use cell::*;
pub use error::*;
pub use grid::*;
pub use metadata::*;
pub use square::*;
pub use util::*;

use crate::text::TxtPuzzle;

#[derive(Debug, Default)]
pub struct TxtReader {
    _strict: bool,
}

impl TxtReader {
    pub fn new(strict: bool) -> Self {
        Self { _strict: strict }
    }

    pub fn read<P>(&self, input: &str) -> Result<P>
    where
        P: TxtPuzzle,
    {
        P::read_text(input)
    }

    pub fn read_from_path<R, P>(&self, path: R) -> Result<P>
    where
        R: AsRef<Path>,
        P: TxtPuzzle,
    {
        let file_str = fs::read_to_string(path)?;
        self.read(&file_str)
    }
}
