use puzzled_core::{Grid, Position};

use crate::{Direction, puzzle::cell::SkyscraperCell};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cells(pub(crate) Grid<SkyscraperCell>);

impl Cells {
    pub fn new(cells: Grid<SkyscraperCell>) -> Self {
        Self(cells)
    }

    pub fn count_visible(&self, line: Position, dir: Direction) -> usize {
        // Iterate over all remaining positions (including the current) in the direction
        let segment = line + dir;
        let iter = self.0.iter_segment(&segment);

        let mut max_height = 0;
        let mut count = 0;

        for cell in iter {
            if let Some(height) = cell.entry()
                && *height > max_height
            {
                max_height = *height;
                count += 1;
            }
        }

        count
    }
}
