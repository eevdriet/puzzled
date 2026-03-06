use ratatui::{
    layout::{HorizontalAlignment, Size, VerticalAlignment},
    style::{Color, Style},
};

#[derive(Debug, Clone, Copy)]
pub struct GridOptions {
    pub cell_width: u16,

    pub cell_height: u16,

    pub inner: Option<Size>,

    pub inner_border_style: Style,
    pub outer_border_style: Style,

    pub draw_inner_borders: bool,

    pub draw_outer_borders: bool,

    pub h_align: HorizontalAlignment,
    pub v_align: VerticalAlignment,
}

impl Default for GridOptions {
    fn default() -> Self {
        Self {
            cell_width: 2,
            cell_height: 1,
            inner: None,
            inner_border_style: Style::default().fg(Color::DarkGray).dim(),
            outer_border_style: Style::default().fg(Color::Yellow),
            draw_inner_borders: true,
            draw_outer_borders: true,
            h_align: HorizontalAlignment::Center,
            v_align: VerticalAlignment::Center,
        }
    }
}
