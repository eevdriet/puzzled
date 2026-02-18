use std::ops;

use crate::Position;

#[derive(Debug, Default, PartialEq)]
pub struct Grid<T> {
    cols: u8,
    rows: u8,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, cols: u8) -> Option<Self> {
        let rows = data.len() / cols as usize;
        let size = rows * usize::from(cols);

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

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get(&self, pos: Position) -> Option<&T> {
        let idx = self.index(pos)?;
        unsafe { Some(self.data.get_unchecked(idx)) }
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        let idx = self.index(pos)?;
        unsafe { Some(self.data.get_unchecked_mut(idx)) }
    }

    fn index(&self, pos: Position) -> Option<usize> {
        if pos.row >= self.rows || pos.col >= self.cols {
            return None;
        }

        Some(usize::from(pos.row) * usize::from(self.cols) + usize::from(pos.col))
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.rows).flat_map(move |row| (0..self.cols).map(move |col| Position { row, col }))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.data.iter_mut()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T> + '_> + '_ {
        (0..self.rows).map(move |row| {
            (0..self.cols).map(move |col| {
                let pos = Position { row, col };
                &self[pos]
            })
        })
    }
}

impl<T> ops::Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, pos: Position) -> &Self::Output {
        let (row, col) = pos.into();
        let (rows, cols) = (self.rows, self.cols);

        self.get(pos).unwrap_or_else(|| {
            panic!("Position is out of bounds: ({row}, {col}) >= ({rows}, {cols})")
        })
    }
}

impl<T> ops::IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        let (row, col) = pos.into();
        let (rows, cols) = (self.rows, self.cols);

        self.get_mut(pos).unwrap_or_else(|| {
            panic!("Position is out of bounds: ({row}, {col}) >= ({rows}, {cols})")
        })
    }
}
