mod builder;
mod cell;
mod clues;
mod grid;
mod iter;
mod line;
mod offset;
mod position;
mod timer;

pub use builder::*;
pub use cell::*;
pub use clues::*;
pub use grid::*;
pub use line::*;
pub use offset::*;
pub use position::*;
pub use timer::*;

#[derive(Debug, Default)]
pub struct Puzzle {
    /// Cells that represent the puzzle grid
    cells: Cells,

    entries: Entries,

    /// Timer that keeps track of the total playing time and whether the user is currently playing
    timer: Timer,

    /// Title of the puzzle
    title: Option<String>,

    /// Author of the puzzle
    author: Option<String>,

    /// Version of the puzzle
    version: Option<String>,

    /// Copyright information
    copyright: Option<String>,

    /// Notes on the puzzle
    notes: Option<String>,
}

impl Puzzle {
    /// Width (number of columns) in the puzzle
    /// Note that this includes blank cells
    pub fn width(&self) -> u8 {
        self.cells.cols()
    }

    /// Width (number of columns) in the puzzle
    /// Note that this includes blank cells
    pub fn height(&self) -> u8 {
        self.cells.rows()
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub fn copyright(&self) -> Option<&str> {
        self.copyright.as_deref()
    }

    pub fn timer(&self) -> &Timer {
        &self.timer
    }

    pub fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }

    pub fn is_solved(&self) -> bool {
        self.cells
            .iter()
            .filter_map(|(_, cell)| match cell {
                Cell::Black => None,
                Cell::Fill(fill) => Some(fill),
            })
            .all(|cell| cell.is_correct())
    }
}
