use ratatui::layout::Rect;

#[derive(Default, Debug, Clone)]
pub struct Viewport {
    pub row_start: u16,
    pub row_end: u16,

    pub col_start: u16,
    pub col_end: u16,

    pub area: Rect,
}

impl Viewport {
    pub fn visible_cols(&self) -> u16 {
        self.col_end - self.col_start
    }

    pub fn visible_rows(&self) -> u16 {
        self.row_end - self.row_start
    }
}
