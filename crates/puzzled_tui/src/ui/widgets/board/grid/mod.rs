mod options;
mod render;

use std::{marker::PhantomData, ops::Range};

pub use options::*;
pub use render::*;
use tui_scrollview::ScrollView;

use crate::{AppContext, AppTypes, CellRender, RenderSize, Widget as AppWidget, render_borders};

use puzzled_core::{Grid, Position};
use ratatui::{
    layout::{Margin, Rect, Size},
    prelude::Buffer,
    widgets::{StatefulWidget, Widget},
};

pub struct GridRefMut<'a, T>(pub &'a mut Grid<T>);

pub struct GridWidget<'a, A: AppTypes, T, C> {
    pub grid: &'a Grid<T>,
    pub cell_state: &'a C,
    _marker: PhantomData<A>,
}

impl<'a, A: AppTypes, T, C> GridWidget<'a, A, T, C> {
    pub fn new(grid: &'a Grid<T>, cell_state: &'a C) -> Self {
        Self {
            grid,
            cell_state,
            _marker: PhantomData,
        }
    }
}

impl<T> RenderSize<GridRenderState> for Grid<T> {
    fn render_size(&self, _area: Rect, opts: &GridRenderState) -> Size {
        opts.size()
    }
}

pub trait RenderGrid<S> {
    fn render(&self, area: Rect, buf: &mut Buffer, render_state: &GridRenderState, cell_state: &S);

    fn visible_rows(&self, render_state: &GridRenderState) -> Range<usize>;
    fn visible_cols(&self, render_state: &GridRenderState) -> Range<usize>;
}

impl<'a, A, T, C> AppWidget<A> for GridWidget<'a, A, T, C>
where
    A: AppTypes,
    T: CellRender<A, C>,
{
    type State = GridRenderState;

    fn render(
        &mut self,
        root: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        state.viewport = root;
        state.rows = self.grid.rows();
        state.cols = self.grid.cols();

        let size = self.render_size(root, ctx, state);
        let mut scroll_view = ScrollView::new(size);

        self.render_all(Rect::from(size), scroll_view.buf_mut(), ctx, state);
        scroll_view.render(root, buf, &mut state.scroll_state);
    }

    fn render_size(&self, _area: Rect, _ctx: &AppContext<A>, state: &Self::State) -> Size {
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

impl<'a, A, T, C> GridWidget<'a, A, T, C>
where
    A: AppTypes,
    T: CellRender<A, C>,
{
    fn render_all(
        &mut self,
        root: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<A>,
        state: &mut GridRenderState,
    ) {
        let area = if state.options.draw_outer_borders {
            root.inner(Margin::new(1, 1))
        } else {
            root
        };

        tracing::trace!(
            "Render grid @ {} ({}x{})",
            state.cursor,
            self.grid.rows(),
            self.grid.cols()
        );
        tracing::trace!("Bordered area: {area:?}");
        tracing::trace!("Area: {area:?}");

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

                let cell_widget = cell.render_cell(pos, self.cell_state, ctx);
                cell_widget.render(cell_area, buf);

                // Draw a background for the whole cell
                x += cell_w;

                if x >= area.right() {
                    break;
                }

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

            if y >= area.bottom() {
                break;
            }

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
                        buf.set_stringn(div_x, y, text, width, state.options.inner_border_style);
                        tracing::trace!("\tDone");
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
            render_borders(area, buf, state);
        }
    }
}
