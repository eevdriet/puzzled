use ratatui::{
    layout::{HorizontalAlignment, Size, VerticalAlignment},
    style::{Color, Style},
};

#[derive(Debug, Clone, Copy)]
pub struct GridOptions {
    pub cell_width: u16,

    pub cell_height: u16,

    pub inner: Option<Size>,

    pub border_style: Style,

    pub draw_inner_borders: bool,

    pub draw_borders: bool,

    pub h_align: HorizontalAlignment,
    pub v_align: VerticalAlignment,
}

impl Default for GridOptions {
    fn default() -> Self {
        Self {
            cell_width: 2,
            cell_height: 1,
            inner: None,
            border_style: Style::default().fg(Color::DarkGray).dim(),
            draw_inner_borders: true,
            draw_borders: true,
            h_align: HorizontalAlignment::Center,
            v_align: VerticalAlignment::Center,
        }
    }
}
