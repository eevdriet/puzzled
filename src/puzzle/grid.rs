use std::ops;

use crate::Position;

#[derive(Debug, PartialEq)]
pub struct Grid<T> {
    cols: u8,
    rows: u8,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, cols: u8) -> Option<Self> {
        let rows = data.len() / cols as usize;
        let size = usize::from(rows) * usize::from(cols);

        if data.len() != size {
            return None;
        }

        Some(Self {
            cols,
            rows: rows as u8,
            data,
        })
    }

    pub fn cols(&self) -> u8 {
        self.cols
    }

    pub fn rows(&self) -> u8 {
        self.rows
    }

    pub fn get(&self, pos: Position) -> Option<&T> {
        let idx = self.index(pos);
        self.data.get(idx)
    }

    fn index(&self, pos: Position) -> usize {
        usize::from(pos.row) * usize::from(self.cols) + usize::from(pos.col)
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.rows).flat_map(move |row| (0..self.cols).map(move |col| Position { row, col }))
    }

    pub fn iter(&self) -> impl Iterator<Item = (Position, &T)> + '_ {
        self.positions().map(move |pos| {
            let val = &self[pos];
            (pos, val)
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> + '_ {
        let cols = self.cols;

        self.data.iter_mut().enumerate().map(move |(i, v)| {
            let row = (i / cols as usize) as u8;
            let col = (i % cols as usize) as u8;
            (Position { row, col }, v)
        })
    }

    pub fn iter_rows(
        &self,
    ) -> impl Iterator<Item = impl Iterator<Item = (Position, &T)> + '_> + '_ {
        (0..self.rows).map(move |row| {
            (0..self.cols).map(move |col| {
                let pos = Position { row, col };
                (pos, &self[pos])
            })
        })
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = impl Iterator<Item = (Position, &T)>> {
        (0..self.cols).map(move |row| {
            (0..self.rows).map(move |col| {
                let pos = Position { row, col };
                (pos, &self[pos])
            })
        })
    }
}

impl<T> ops::Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Position) -> &Self::Output {
        let idx = self.index(pos);
        &self.data[idx]
    }
}

impl<T> ops::IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        let idx = self.index(pos);
        &mut self.data[idx]
    }
}
