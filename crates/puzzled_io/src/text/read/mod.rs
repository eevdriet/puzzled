mod error;
mod grid;
mod metadata;
mod state;

use std::{fs, path::Path};

pub use error::*;
pub use state::*;

use crate::text::TxtPuzzle;

#[derive(Debug, Default)]
pub struct TxtReader {
    strict: bool,
}

impl TxtReader {
    pub fn new(strict: bool) -> Self {
        Self { strict }
    }

    pub fn read<P>(&self, input: &str) -> Result<P>
    where
        P: TxtPuzzle,
    {
        let mut state = TxtState::new(input, self.strict);
        let puzzle = P::from_text(&mut state)?;

        Ok(puzzle)
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
