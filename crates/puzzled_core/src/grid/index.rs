use std::ops;

use crate::{Grid, Position};

impl<T> Grid<T> {
    pub fn get<P>(&self, pos: P) -> Option<&T>
    where
        P: Into<Position>,
    {
        let idx = self.index(pos.into())?;
        unsafe { Some(self.data.get_unchecked(idx)) }
    }

    pub fn get_mut<P>(&mut self, pos: P) -> Option<&mut T>
    where
        P: Into<Position>,
    {
        let idx = self.index(pos.into())?;
        unsafe { Some(self.data.get_unchecked_mut(idx)) }
    }

    pub fn index(&self, pos: Position) -> Option<usize> {
        if pos.row >= self.rows || pos.col >= self.cols {
            return None;
        }

        Some(pos.row * self.cols + pos.col)
    }

    pub fn position(&self, idx: usize) -> Option<Position> {
        if idx >= self.data.len() {
            return None;
        }

        let row = idx / self.cols;
        let col = idx % self.cols;

        Some(Position::new(row, col))
    }
}

impl<T, P> ops::Index<P> for Grid<T>
where
    P: Into<Position>,
{
    type Output = T;

    fn index(&self, pos: P) -> &Self::Output {
        let pos: Position = pos.into();

        self.get(pos).unwrap_or_else(|| {
            let (row, col) = (pos.row, pos.col);
            let (rows, cols) = (self.rows, self.cols);

            panic!("Position is out of bounds: ({row}, {col}) >= ({rows}, {cols})")
        })
    }
}

impl<T, P> ops::IndexMut<P> for Grid<T>
where
    P: Into<Position>,
{
    fn index_mut(&mut self, pos: P) -> &mut Self::Output {
        let pos: Position = pos.into();
        let rows = self.rows;
        let cols = self.cols;

        self.get_mut(pos).unwrap_or_else(|| {
            let (row, col) = (pos.row, pos.col);

            panic!("Position is out of bounds: ({row}, {col}) >= ({rows}, {cols})")
        })
    }
}
