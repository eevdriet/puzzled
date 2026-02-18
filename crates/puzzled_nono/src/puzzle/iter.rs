use crate::{Fill, Line, Position, Puzzle, Runs};

impl Puzzle {
    pub fn iter_line<'a>(&'a self, line: Line) -> LineIter<'a> {
        match line {
            Line::Row(row) if row < self.rows => {
                let start = usize::from(row) * usize::from(self.cols);
                let end = start + usize::from(self.cols);

                let iter = self.fills[start..end].iter();
                LineIter::Row(iter)
            }
            Line::Col(col) if col < self.cols => LineIter::Col(ColIter {
                puzzle: self,
                col,
                front: 0,
                back: self.rows,
                rows: self.rows,
            }),
            _ => LineIter::Empty,
        }
    }

    pub fn iter_rows<'a>(&'a self) -> impl Iterator<Item = LineIter<'a>> {
        (0..self.rows).map(move |row| self.iter_line(Line::Row(row)))
    }

    pub fn iter_cols<'a>(&'a self) -> impl Iterator<Item = LineIter<'a>> {
        (0..self.cols).map(move |col| self.iter_line(Line::Col(col)))
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = &Fill> {
        self.fills.iter()
    }

    pub fn iter_runs<'a>(&'a self, line: Line) -> Runs<LineIter<'a>> {
        let fills = self.iter_line(line);
        Runs::new(fills, true)
    }
}

#[derive(Debug, Clone)]
pub enum LineIter<'a> {
    Row(std::slice::Iter<'a, Fill>),
    Col(ColIter<'a>),
    Empty,
}

impl<'a> Iterator for LineIter<'a> {
    type Item = &'a Fill;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Row(iter) => iter.next(),
            Self::Col(iter) => iter.next(),
            _ => None,
        }
    }
}

impl<'a> DoubleEndedIterator for LineIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Row(iter) => iter.next_back(),
            Self::Col(iter) => iter.next_back(),
            _ => None,
        }
    }
}

impl<'a> ExactSizeIterator for LineIter<'a> {
    fn len(&self) -> usize {
        match self {
            Self::Row(iter) => iter.len(),
            Self::Col(iter) => iter.len(),
            Self::Empty => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColIter<'a> {
    puzzle: &'a Puzzle,
    col: u16,

    // Current position within the iterator
    front: u16,
    back: u16,

    // How many rows there are (needed for ExactSizeIterator)
    rows: u16,
}

impl<'a> Iterator for ColIter<'a> {
    type Item = &'a Fill;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        let pos = Position {
            row: self.front,
            col: self.col,
        };
        self.front += 1;

        Some(&self.puzzle[pos])
    }
}

impl<'a> DoubleEndedIterator for ColIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        self.back -= 1;
        let pos = Position {
            row: self.back,
            col: self.col,
        };

        Some(&self.puzzle[pos])
    }
}

impl<'a> ExactSizeIterator for ColIter<'a> {
    fn len(&self) -> usize {
        self.front.abs_diff(self.back) as usize
    }
}
