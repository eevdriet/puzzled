mod cell;
mod grid;
mod sided;
mod viewport;

pub use cell::*;
pub use grid::*;
pub use sided::*;
pub use viewport::*;

use puzzled_core::Position;
use ratatui::{
    prelude::{Buffer, Rect},
    text::Text,
    widgets::Widget,
};

pub trait CellRender<S> {
    fn render_cell(&self, pos: Position, state: &S) -> impl Widget;
}
pub trait LineRender<S> {
    fn render_row(&self, row: usize, state: &S) -> Text<'_>;

    fn render_col(&self, col: usize, state: &S) -> Text<'_>;
}

fn render_borders(area: Rect, buf: &mut Buffer, state: &GridRenderState) {
    let style = state.options.outer_border_style;

    // Corners
    let x_start = area.x;
    let y_start = area.y;
    let x_end = area.right() - 1;
    let y_end = area.bottom() - 1;

    // Top and bottom borders
    for x in x_start..=x_end {
        buf.set_string(x, y_start, "─", style);
    }
    for x in x_start..=x_end {
        buf.set_string(x, y_end, "─", style);
    }

    // Left and right borders
    for y in y_start..=y_end {
        buf.set_string(x_start, y, "│", style);
    }
    for y in y_start..=y_end {
        buf.set_string(x_end, y, "│", style);
    }

    // Corners
    buf.set_string(x_start, y_start, "┌", style);
    buf.set_string(x_end, y_start, "┐", style);
    buf.set_string(x_start, y_end, "└", style);
    buf.set_string(x_end, y_end, "┘", style);

    // Inner borders
    if state.options.draw_outer_borders
        && let Some(size) = state.options.inner_borders
    {
        let w = state.options.cell_width * size.width + 1;
        let h = state.options.cell_height * size.height + 1;

        // Top / bottom
        for x in (x_start + w..area.right() - 1).step_by(w as usize) {
            buf.set_string(x, y_start, "┬", style);
            buf.set_string(x, y_end, "┴", style);
        }

        // Left / right
        for y in (y_start + h..area.bottom() - 1).step_by(h as usize) {
            buf.set_string(x_start, y, "├", style);
            buf.set_string(x_end, y, "┤", style);
        }
    }
}
