use puzzled_core::{Grid, Position, Value};

use crate::{Direction, Skyscraper};

pub trait Cells {
    fn count_visible(&self, pos: Position, dir: Direction) -> usize;
}

impl<T> Cells for Grid<T>
where
    T: Value<Skyscraper>,
{
    fn count_visible(&self, pos: Position, dir: Direction) -> usize {
        // Iterate over all remaining positions (including the current) in the direction
        let iter = self.iter_segment(pos, dir);

        let mut max_height = 0;
        let mut count = 0;

        for cell in iter {
            if let Some(skyscraper) = cell.value()
                && skyscraper.height() > max_height
            {
                max_height = skyscraper.height();
                count += 1;
            }
        }

        count
    }
}
