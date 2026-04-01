mod cell;
mod grid;
mod sided;
mod viewport;

pub use cell::*;
pub use grid::*;
pub use sided::*;
pub use viewport::*;

use puzzled_core::{Line, Position};
use ratatui::prelude::{Buffer, Rect};

use crate::{AppCommand, AppContext, AppTypes};

pub trait CellRender<A: AppTypes, C> {
    fn render_cell(
        &self,
        pos: Position,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<A>,
        render: &GridRenderState,
        state: &C,
    );

    fn on_command(
        &self,
        _command: AppCommand<A>,
        _pos: Position,
        _ctx: &mut AppContext<A>,
        _render: &mut GridRenderState,
        _state: &mut C,
    ) -> bool {
        false
    }
}
pub trait EdgeRender<A: AppTypes, E> {
    fn render_row(
        &self,
        row: usize,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<A>,
        render: &SidedGridRenderState,
        state: &E,
    );

    fn render_col(
        &self,
        col: usize,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<A>,
        render: &SidedGridRenderState,
        state: &E,
    );

    fn height(
        &self,
        area: Rect,
        ctx: &AppContext<A>,
        render: &SidedGridRenderState,
        state: &E,
    ) -> u16;

    fn width(
        &self,
        area: Rect,
        ctx: &AppContext<A>,
        render: &SidedGridRenderState,
        state: &E,
    ) -> u16;

    fn on_command(
        &mut self,
        _command: AppCommand<A>,
        _line: Line,
        _ctx: &mut AppContext<A>,
        _state: &mut E,
    ) -> bool {
        false
    }
}

impl<A: AppTypes, E> EdgeRender<A, E> for () {
    fn render_row(
        &self,
        _row: usize,
        _area: Rect,
        _buf: &mut Buffer,
        _ctx: &AppContext<A>,
        _render: &SidedGridRenderState,
        _state: &E,
    ) {
    }
    fn render_col(
        &self,
        _col: usize,
        _area: Rect,
        _buf: &mut Buffer,
        _ctx: &AppContext<A>,
        _render: &SidedGridRenderState,
        _state: &E,
    ) {
    }

    fn height(
        &self,
        _area: Rect,
        _ctx: &AppContext<A>,
        _render: &SidedGridRenderState,
        _state: &E,
    ) -> u16 {
        0
    }

    fn width(
        &self,
        _area: Rect,
        _ctx: &AppContext<A>,
        _render: &SidedGridRenderState,
        _state: &E,
    ) -> u16 {
        0
    }
}

pub fn render_borders(area: Rect, buf: &mut Buffer, state: &GridRenderState) {
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
