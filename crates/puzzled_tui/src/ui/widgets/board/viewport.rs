use puzzled_core::Grid;
use ratatui::layout::Rect;

#[derive(Default, Debug, Clone, Copy)]
pub struct Viewport {
    // Grid specific
    pub row_start: usize,
    pub row_end: usize,

    pub col_start: usize,
    pub col_end: usize,

    // Terminal specific
    pub area: Rect,
}

impl Viewport {
    pub fn from_grid<T>(grid: &Grid<T>) -> Self {
        Self {
            row_start: 0,
            col_start: 0,
            row_end: grid.rows(),
            col_end: grid.cols(),
            area: Rect::default(),
        }
    }

    pub fn rows(&self) -> usize {
        self.row_end - self.row_start
    }

    pub fn cols(&self) -> usize {
        self.col_end - self.col_start
    }
}
