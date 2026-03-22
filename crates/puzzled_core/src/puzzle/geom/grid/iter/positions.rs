use derive_more::Debug;

use crate::{Grid, Position};

#[derive(Debug)]
pub struct GridPositionsIter<'a, T> {
    #[debug(skip)]
    pub(crate) grid: &'a Grid<T>,
    pub(crate) positions: Vec<Position>,

    pub(crate) front: usize,
    pub(crate) back: usize,
}

impl<'a, T> GridPositionsIter<'a, T> {
    pub fn new(grid: &'a Grid<T>, positions: Vec<Position>) -> Self {
        Self {
            grid,
            front: 0,
            back: positions.len(),
            positions,
        }
    }
}

impl<'a, T> Iterator for GridPositionsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        let pos = self.positions[self.front];
        self.front += 1;

        self.grid.get(pos)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.back - self.front;
        (remaining, Some(remaining))
    }
}

impl<'a, T> DoubleEndedIterator for GridPositionsIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back = self.back.saturating_sub(1);

        if self.front >= self.back {
            return None;
        }

        let pos = self.positions[self.back];

        self.grid.get(pos)
    }
}

impl<'a, T> ExactSizeIterator for GridPositionsIter<'a, T> {
    fn len(&self) -> usize {
        self.back - self.front
    }
}

impl<'a, T> Clone for GridPositionsIter<'a, T> {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            front: self.front,
            back: self.back,

            positions: self.positions.clone(),
        }
    }
}
