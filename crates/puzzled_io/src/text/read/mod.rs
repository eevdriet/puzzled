mod cell;
mod error;
mod grid;
mod metadata;
mod state;
mod util;

use std::{fs, path::Path};

pub use error::*;
pub use state::*;
pub use util::*;

use crate::text::TxtPuzzle;

#[derive(Debug, Default)]
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
        let mut state = TxtState::new(input, self.strict);

        P::read_text(&mut state)
    }

    pub fn read_from_path<R, P, S>(&self, path: R) -> Result<(P, S)>
    where
        R: AsRef<Path>,
        P: TxtPuzzle<S>,
    {
        let file_str = fs::read_to_string(path)?;
        self.read(&file_str)
    }
}
