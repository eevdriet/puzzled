use puzzled_core::{Grid, Line, LineIter, Position};
use std::ops;

use crate::{Fill, Nonogram, Runs};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Fills(Grid<Fill>);

impl Fills {
    pub fn new(fills: Grid<Fill>) -> Self {
        Self(fills)
    }

    pub fn iter_line_runs<'a>(&'a self, line: Line) -> Runs<LineIter<'a, Fill>> {
        let fills = self.iter_line(line);
        Runs::new(fills, true)
    }
}

impl ops::Deref for Fills {
    type Target = Grid<Fill>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Fills {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ops::Index<Position> for Nonogram {
    type Output = Fill;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.fills[pos]
    }
}

impl ops::IndexMut<Position> for Nonogram {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.fills[pos]
    }
}
