mod cell;
mod clue;
mod skyscraper;

use std::fmt;

pub use cell::*;
pub use clue::*;
pub use skyscraper::*;

use puzzled_core::{Cell, Direction, Grid, Line, Metadata, Puzzle, SidedGridDisplay};

#[derive(Debug, PartialEq, Eq)]
pub struct Skyscrapers {
    // State
    cells: Grid<Cell<Skyscraper>>,
    clues: Clues,

    // Metadata
    meta: Metadata,
}

impl Skyscrapers {
    pub fn new(cells: Grid<Cell<Skyscraper>>, clues: Clues, meta: Metadata) -> Self {
        Self { cells, clues, meta }
    }

    pub fn cells(&self) -> &Grid<Cell<Skyscraper>> {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut Grid<Cell<Skyscraper>> {
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

    type Solution = Grid<Skyscraper>;
}

impl fmt::Display for Skyscrapers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cells = self.cells();
        let clues = self.clues();

        let rows = cells.rows();
        let cols = cells.cols();

        let display_side = |direction: Direction| {
            let len = match direction {
                Direction::Up | Direction::Down => cols,
                _ => rows,
            };

            (0..len)
                .map(|pos| {
                    let line = match direction {
                        Direction::Down => Line::Col(pos),
                        Direction::Left => Line::Row(pos),
                        Direction::Up => Line::Col(pos),
                        Direction::Right => Line::Row(pos),
                    };

                    let key = (line, direction).into();
                    clues.get(&key)
                })
                .collect::<Vec<_>>()
        };

        let left = display_side(Direction::Right);
        let right = display_side(Direction::Left);
        let top = display_side(Direction::Down);
        let bottom = display_side(Direction::Up);

        write!(
            f,
            "{}",
            SidedGridDisplay {
                grid: self.cells(),
                left: &left,
                right: &right,
                top: &top,
                bottom: &bottom,
            }
        )?;

        write!(f, "{}", self.meta())?;

        Ok(())
    }
}
