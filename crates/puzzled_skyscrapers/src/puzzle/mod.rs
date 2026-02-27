mod cell;
mod clue;

pub use cell::*;
pub use clue::*;
use puzzled_core::{Grid, Metadata, Puzzle};

#[derive(Debug, PartialEq, Eq)]
pub struct Skyscrapers {
    // State
    cells: Cells,
    clues: Clues,

    // Metadata
    meta: Metadata,
}

impl Skyscrapers {
    pub fn new(cells: Cells, clues: Clues, meta: Metadata) -> Self {
        Self { cells, clues, meta }
    }

    pub fn cells(&self) -> &Cells {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut Cells {
        &mut self.cells
    }

    pub fn clues(&self) -> &Clues {
        &self.clues
    }

    pub fn clues_mut(&mut self) -> &mut Clues {
        &mut self.clues
    }

    pub fn meta(&self) -> &Metadata {
        &self.meta
    }
}

impl Puzzle for Skyscrapers {
    const NAME: &'static str = "Skyscrapers";

    type Solution = Grid<u8>;
}
