use crate::{Line, Position};

mod error;
mod index;
mod iter;

pub use error::Error as GridError;
pub use iter::*;

#[derive(Debug, Default)]
pub struct Grid<T> {
    cols: usize,
    rows: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    /// Create a new grid filled with the result of the given function
    ///
    /// Returns [`None`] if the size overflows, i.e. when `rows * cols >= usize::MAX`
    pub fn new_with<F>(rows: usize, cols: usize, value_fn: F) -> Result<Self, GridError>
    where
        F: FnMut() -> T,
    {
        let size = match rows.checked_mul(cols) {
            Some(size) => size,
            None => return Err(GridError::SizeOverflow { rows, cols }),
        };

        let mut data = Vec::with_capacity(size);
        data.fill_with(value_fn);
        Ok(Self { rows, cols, data })
    }

    /// Create a grid from a data [`Vec<T>`] and the number of columns to use
    ///
    /// Returns [`None`] if the data does not divide the number of columns
    pub fn from_vec(data: Vec<T>, cols: usize) -> Result<Self, GridError> {
        let len = data.len();
        if !len.is_multiple_of(cols) {
            return Err(GridError::ColDivisibility { len, cols });
        }

        let rows = data.len() / cols;
        Ok(Self { cols, rows, data })
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
    T: Clone,
{
    /// Create a new grid filled with the given value
    ///
    /// Returns [`None`] if the size overflows, i.e. when `rows * cols >= usize::MAX`
    pub fn new_from(rows: usize, cols: usize, value: T) -> Option<Self> {
        let size = rows.checked_mul(cols)?;

        let data = vec![value; size];
        Some(Self { rows, cols, data })
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    /// Create a new grid filled with [`T::Default`][Default]
    ///
    /// Returns [`None`] if the size overflows, i.e. when `rows * cols >= usize::MAX`
    pub fn new(rows: usize, cols: usize) -> Result<Self, GridError> {
        Self::new_with(rows, cols, T::default)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Deserialize, Serialize, ser::SerializeStruct};

    use crate::Grid;

    #[derive(Deserialize)]
    #[serde(rename_all = "lowercase")]
    struct SerdeGrid<T> {
        cols: usize,
        rows: usize,
        data: Vec<T>,
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<T: Serialize> Serialize for Grid<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut grid = serializer.serialize_struct("Grid", 3)?;
            grid.serialize_field("cols", &self.cols)?;
            grid.serialize_field("rows", &self.rows)?;
            grid.serialize_field("data", &self.data)?;

            grid.end()
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de, T: Deserialize<'de>> Deserialize<'de> for Grid<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let SerdeGrid { cols, rows, data } = SerdeGrid::deserialize(deserializer)?;
            let grid = Grid { cols, rows, data };

            Ok(grid)
        }
    }
}
