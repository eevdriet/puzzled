use crate::{Line, Position};

mod index;
mod iter;
mod macros;

pub use iter::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default)]
pub struct Grid<T> {
    cols: usize,
    rows: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    /// Create a grid from a data [`Vec<T>`] and the number of columns to use
    ///
    /// Returns [`None`] if the data does not divide the number of columns
    pub fn from_vec(data: Vec<T>, cols: usize) -> Option<Self> {
        if !data.len().is_multiple_of(cols) {
            return None;
        }

        let rows = data.len() / cols;
        Some(Self { cols, rows, data })
    }

    /// Number of columns in the grid
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Number of rows in the grid
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn size(&self) -> usize {
        self.cols * self.rows
    }

    /// Reference the underlying data [`Vec`]
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Map each entry in the grid to create a new grid
    pub fn map<U, F>(self, f: F) -> Grid<U>
    where
        F: FnMut(T) -> U,
    {
        Grid {
            data: self.data.into_iter().map(f).collect(),
            cols: self.cols,
            rows: self.rows,
        }
    }

    /// Map each referenced entry in the grid to create a new grid
    pub fn map_ref<U, F>(&self, f: F) -> Grid<U>
    where
        F: FnMut(&T) -> U,
    {
        Grid {
            data: self.data.iter().map(f).collect(),
            cols: self.cols,
            rows: self.rows,
        }
    }

    /// Map each indexed entry in the grid to create a new grid
    pub fn map_indexed<U, F>(self, mut f: F) -> Grid<U>
    where
        F: FnMut(Position, T) -> U,
    {
        let cols = self.cols;
        let data = self
            .data
            .into_iter()
            .enumerate()
            .map(|(idx, val)| {
                let pos = Position::from_row_order(idx, cols);
                f(pos, val)
            })
            .collect();

        Grid {
            data,
            cols,
            rows: self.rows,
        }
    }

    /// Map each indexed referenced entry in the grid to create a new grid
    pub fn map_ref_indexed<U, F>(&self, mut f: F) -> Grid<U>
    where
        F: FnMut(Position, &T) -> U,
    {
        let cols = self.cols;
        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(idx, val)| {
                let pos = Position::from_row_order(idx, cols);
                f(pos, val)
            })
            .collect();

        Grid {
            data,
            cols,
            rows: self.rows,
        }
    }

    /// Try to swap the elements at the given [positions](Position)
    ///
    /// If either position is out of bounds, [`None`] is returned.
    /// Otherwise, [`Some<bool>`] is returned indicating whether a swap was performed.
    /// ```
    /// use puzzled_core::{grid, Position};
    ///
    /// let mut grid = grid![
    ///     [1, 2],
    ///     [3, 4],
    /// ];
    ///
    /// let pos1 = Position::new(0, 0);
    /// let pos2 = Position::new(1, 1);
    /// let pos3 = Position::new(2, 2);
    ///
    /// assert_eq!(grid.swap(pos1, pos1), Some(false));
    /// assert_eq!(grid.swap(pos1, pos2), Some(true));
    /// assert_eq!(grid.swap(pos1, pos3), None);
    /// assert_eq!(grid[pos1], 4);
    /// ```
    pub fn swap(&mut self, pos1: Position, pos2: Position) -> Option<bool> {
        let idx1 = self.index(pos1)?;
        let idx2 = self.index(pos2)?;

        if pos1 == pos2 {
            return Some(false);
        }

        self.data.swap(idx1, idx2);
        Some(true)
    }

    pub fn line_len(&self, line: Line) -> usize {
        match line {
            Line::Row(_) => self.cols,
            Line::Col(_) => self.rows,
        }
    }
}

impl<T> PartialEq for Grid<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows {
            return false;
        }
        if self.cols != other.cols {
            return false;
        }

        self.data == other.data
    }
}

impl<T> Eq for Grid<T> where T: Eq {}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Grid {
            cols: self.cols,
            rows: self.rows,
            data: self.data.clone(),
        }
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    /// Create a new grid with a given size with [`T::Default`][Default]
    ///
    /// Returns [`None`] if the size overflows, i.e. when `rows * cols >= usize::MAX`
    pub fn new(rows: usize, cols: usize) -> Option<Self> {
        let size = rows.checked_mul(cols)?;

        let mut data = Vec::with_capacity(size);
        data.fill_with(T::default);

        Some(Self { rows, cols, data })
    }
}
