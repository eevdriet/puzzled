mod options;
mod state;

use std::ops::Range;

pub use options::*;
pub use state::*;

use puzzled_core::{Grid, Position};
use ratatui::{
    layout::{Margin, Rect, Size},
    prelude::Buffer,
    widgets::{StatefulWidget, Widget},
};

use crate::{
    AppTypes, CellRender, RenderSize, Widget as AppWidget, widgets::board::render_borders,
};

pub struct GridRefMut<'a, T>(pub &'a mut Grid<T>);

pub struct GridWidget<'a, T, S> {
    pub grid: &'a Grid<T>,
    pub cell_state: &'a S,
}

impl<'a, T, S> GridWidget<'a, T, S> {
    pub fn new(grid: &'a Grid<T>, cell_state: &'a S) -> Self {
        Self { grid, cell_state }
    }
}

impl<T> RenderSize<GridOptions> for Grid<T> {
    fn render_size(&self, _area: Rect, opts: &GridOptions) -> Size {
        let cols = self.cols() as u16;
        let rows = self.rows() as u16;

        let mut width = cols * opts.cell_width;
        let mut height = rows * opts.cell_height;

        if let Some(inner) = opts.inner_borders {
            width += (cols - 1) / inner.width;
            height += (rows - 1) / inner.height;
        }

        if opts.draw_outer_borders {
            width += 2;
            height += 2;
        }

        Size { width, height }
    }
}

pub trait RenderGrid<S> {
    fn render(&self, area: Rect, buf: &mut Buffer, render_state: &GridRenderState, cell_state: &S);

    fn visible_rows(&self, render_state: &GridRenderState) -> Range<usize>;
    fn visible_cols(&self, render_state: &GridRenderState) -> Range<usize>;
}

impl<'a, A, T, S> AppWidget<A> for GridWidget<'a, T, S>
where
    A: AppTypes,
    T: CellRender<S>,
{
    type State = GridRenderState;

    fn render(&mut self, bordered_area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = if state.options.draw_outer_borders {
            bordered_area.inner(Margin::new(1, 1))
        } else {
            bordered_area
        };
        tracing::info!("Bordered area: {bordered_area:?}");
        tracing::info!("Area: {area:?}");

        // Render the grid itself
        let opts = &state.options;

        let draw_inner = opts.draw_inner_borders;
        let cell_w = opts.cell_width;
        let cell_h = opts.cell_height;

        let x_start = area.x;
        let mut y = area.y;

        for row in 0..self.grid.rows() {
            let mut x = x_start;

            for col in 0..self.grid.cols() {
                // Draw the value at the current position in the grid
                let pos = Position::new(row, col);
                let cell = &self.grid[pos];
                tracing::trace!("Drawing at {pos:?} [({x}, {y}) on screen]");
                let cell_area = Rect::new(x, y, cell_w, cell_h);

                let cell_widget = cell.render_cell(pos, self.cell_state);
                cell_widget.render(cell_area, buf);

                // Draw a background for the whole cell

                x += cell_w;

                // Draw inner vertical border if defined
                let col = col as u16;

                if let Some(size) = state.options.inner_borders
                    && (col + 1).is_multiple_of(size.width)
                {
                    if draw_inner {
                        for div_y in y..y + cell_h {
                            tracing::trace!("\tSetting │ at ({x}, {div_y}) on screen");
                            buf.set_stringn(x, div_y, "│", 1, state.options.inner_border_style);
                            tracing::trace!("\tDone");
                        }
                    }

                    x += 1;
                }
            }

            // Advance y by cell height
            y += cell_h;

            // Draw horizontal divider
            let row = row as u16;

            if let Some(size) = state.options.inner_borders
                && (row + 1).is_multiple_of(size.height)
            {
                let mut div_x = x_start;

                for col in 0..self.grid.cols() {
                    let col = col as u16;

                    if draw_inner {
                        let width = (area.right() - 1).saturating_sub(div_x).min(cell_w) as usize;
                        let text = "─".repeat(width);

                        tracing::trace!(
                            "\tSetting ─ at ({div_x}..{}, {y}) on screen",
                            div_x + width as u16
                        );
                        tracing::trace!("\tDone");
                        buf.set_stringn(div_x, y, text, width, state.options.inner_border_style);
                    }

                    div_x += cell_w;

                    if (col + 1).is_multiple_of(size.width) {
                        if draw_inner && y < area.bottom() {
                            tracing::trace!("\tSetting ┼ at ({div_x}, {y}) on screen");
                            buf.set_stringn(div_x, y, "┼", 1, state.options.inner_border_style);
                            tracing::trace!("\tDone");
                        }

                        div_x += 1;
                    }
                }

                y += 1;
            }
        }

        // Render borders
        if state.options.draw_outer_borders {
            render_borders(bordered_area, buf, state);
        }
    }

    fn render_size(&self, _area: Rect, state: &Self::State) -> Size {
        let cols = self.grid.cols() as u16;
        let rows = self.grid.rows() as u16;

        let mut width = cols * state.options.cell_width;
        let mut height = rows * state.options.cell_height;

        if let Some(inner) = state.options.inner_borders {
            width += (cols - 1) / inner.width;
            height += (rows - 1) / inner.height;
        }

        if state.options.draw_outer_borders {
            width += 2;
            height += 2;
        }

        Size { width, height }
    }
}

impl<'a, T, S> StatefulWidget for GridWidget<'a, T, S>
where
    T: CellRender<S>,
{
    type State = GridRenderState;

    fn render(self, bordered_area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let area = if state.options.draw_outer_borders {
            bordered_area.inner(Margin::new(1, 1))
        } else {
            bordered_area
        };
        tracing::info!("Bordered area: {bordered_area:?}");
        tracing::info!("Area: {area:?}");

        // Render the grid itself
        let opts = &state.options;

        let draw_inner = opts.draw_inner_borders;
        let cell_w = opts.cell_width;
        let cell_h = opts.cell_height;

        let x_start = area.x;
        let mut y = area.y;

        for row in 0..self.grid.rows() {
            let mut x = x_start;

            for col in 0..self.grid.cols() {
                // Draw the value at the current position in the grid
                let pos = Position::new(row, col);
                let cell = &self.grid[pos];
                tracing::trace!("Drawing at {pos:?} [({x}, {y}) on screen]");
                let cell_area = Rect::new(x, y, cell_w, cell_h);

                let cell_widget = cell.render_cell(pos, self.cell_state);
                cell_widget.render(cell_area, buf);

                // Draw a background for the whole cell

                x += cell_w;

                // Draw inner vertical border if defined
                let col = col as u16;

                if let Some(size) = state.options.inner_borders
                    && (col + 1).is_multiple_of(size.width)
                {
                    if draw_inner {
                        for div_y in y..y + cell_h {
                            tracing::trace!("\tSetting │ at ({x}, {div_y}) on screen");
                            buf.set_stringn(x, div_y, "│", 1, state.options.inner_border_style);
                            tracing::trace!("\tDone");
                        }
                    }

                    x += 1;
                }
            }

            // Advance y by cell height
            y += cell_h;

            // Draw horizontal divider
            let row = row as u16;

            if let Some(size) = state.options.inner_borders
                && (row + 1).is_multiple_of(size.height)
            {
                let mut div_x = x_start;

                for col in 0..self.grid.cols() {
                    let col = col as u16;

                    if draw_inner {
                        let width = (area.right() - 1).saturating_sub(div_x).min(cell_w) as usize;
                        let text = "─".repeat(width);

                        tracing::trace!(
                            "\tSetting ─ at ({div_x}..{}, {y}) on screen",
                            div_x + width as u16
                        );
                        tracing::trace!("\tDone");
                        buf.set_stringn(div_x, y, text, width, state.options.inner_border_style);
                    }

                    div_x += cell_w;

                    if (col + 1).is_multiple_of(size.width) {
                        if draw_inner && y < area.bottom() {
                            tracing::trace!("\tSetting ┼ at ({div_x}, {y}) on screen");
                            buf.set_stringn(div_x, y, "┼", 1, state.options.inner_border_style);
                            tracing::trace!("\tDone");
                        }

                        div_x += 1;
                    }
                }

                y += 1;
            }
        }

        // Render borders
        if state.options.draw_outer_borders {
            render_borders(bordered_area, buf, state);
        }
    }
}
