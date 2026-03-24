mod bit;

use std::fmt;

pub use bit::*;

use derive_more::{Index, IndexMut};
use puzzled_core::{Cell, Grid, Metadata, Puzzle};

#[derive(Debug, Clone, PartialEq, Eq, Index, IndexMut)]
pub struct Binario {
    // State
    #[index]
    #[index_mut]
    cells: Grid<Cell<Bit>>,

    // Metadata
    meta: Metadata,
}

impl Binario {
    pub fn new(cells: Grid<Cell<Bit>>, meta: Metadata) -> Self {
        Self { cells, meta }
    }

    pub fn cells(&self) -> &Grid<Cell<Bit>> {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut Grid<Cell<Bit>> {
        &mut self.cells
    }

    pub fn meta(&self) -> &Metadata {
        &self.meta
    }
}

impl fmt::Display for Binario {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.cells)?;
        writeln!(f, "{}", self.meta)?;

        Ok(())
    }
}

impl Puzzle for Binario {
    const NAME: &'static str = "Binario";

    type Solution = Grid<Bit>;
    type Position = puzzled_core::Position;
    type Value = Bit;
}
