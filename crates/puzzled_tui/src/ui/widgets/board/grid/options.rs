use ratatui::{
    layout::{HorizontalAlignment, Size, VerticalAlignment},
    style::{Color, Style},
};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(default)]
pub struct GridOptions {
    pub cell_width: u16,

    pub cell_height: u16,

    pub inner_borders: Option<Size>,

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
            inner_borders: None,
            inner_border_style: Style::default().fg(Color::White),
            outer_border_style: Style::default().fg(Color::White),
            draw_inner_borders: false,
            draw_outer_borders: false,
            h_align: HorizontalAlignment::Center,
            v_align: VerticalAlignment::Center,
        }
    }
}
