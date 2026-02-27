mod bit;

pub use bit::*;

use puzzled_core::{Cell, Grid, Metadata, Puzzle};

#[derive(Debug, PartialEq, Eq)]
pub struct Binario {
    // State
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

impl Puzzle for Binario {
    type Solution = Grid<Bit>;
}
