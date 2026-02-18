mod index;
mod iter;

#[derive(Debug, Default, PartialEq)]
pub struct Grid<T> {
    cols: usize,
    rows: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_vec(data: Vec<T>, cols: usize) -> Option<Self> {
        if !data.len().is_multiple_of(cols) {
            return None;
        }

        let rows = data.len() / cols;
        Some(Self { cols, rows, data })
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn new(rows: usize, cols: usize) -> Option<Self> {
        let size = rows.checked_mul(cols)?;

        let mut data = Vec::with_capacity(size);
        data.fill_with(T::default);

        Some(Self { rows, cols, data })
    }
}
