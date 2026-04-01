mod options;
mod render;
mod state;

use std::marker::PhantomData;

pub use options::*;
pub use render::*;
pub use state::*;

use tui_scrollview::ScrollbarVisibility;

use crate::{AppContext, AppTypes, CellRender, ScrollWidget, Widget as AppWidget, render_borders};

use puzzled_core::{Grid, Position};
use ratatui::{
    layout::{Margin, Rect, Size},
    prelude::Buffer,
};

pub struct GridRefMut<'a, T>(pub &'a mut Grid<T>);

pub struct GridWidget<'a, A: AppTypes, T, C> {
    pub grid: &'a Grid<T>,

    _app: PhantomData<A>,
    _cell: PhantomData<C>,
}

impl<'a, A: AppTypes, T, C> GridWidget<'a, A, T, C> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self {
            grid,
            _app: PhantomData,
            _cell: PhantomData,
        }
    }
}

impl<'a, A, T, C> AppWidget<A> for GridWidget<'a, A, T, C>
where
    A: AppTypes,
    T: CellRender<A, C>,
    C: 'a,
{
    type State = GridWidgetState<'a, C>;

    fn render(
        &mut self,
        root: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        state.render.viewport = root;
        state.render.rows = self.grid.rows();
        state.render.cols = self.grid.cols();

        let size = self.render_size(root, ctx, state);
        let area = self.render_area(root, ctx, state);

        let mut scroll_view =
            ScrollWidget::new(size).scrollbars_visibility(ScrollbarVisibility::Never);

        self.render_all(Rect::from(size), scroll_view.buf_mut(), ctx, state);
        scroll_view.render(area, buf, ctx, &mut state.render.scroll_state);
    }

    fn render_size(&self, _area: Rect, _ctx: &AppContext<A>, state: &mut Self::State) -> Size {
        let cols = self.grid.cols() as u16;
        let rows = self.grid.rows() as u16;

        let mut width = cols * state.render.options.cell_width;
        let mut height = rows * state.render.options.cell_height;

        if let Some(inner) = state.render.options.inner_borders {
            width += (cols - 1) / inner.width;
            height += (rows - 1) / inner.height;
        }

        if state.render.options.draw_outer_borders {
            width += 2;
            height += 2;
        }

        Size { width, height }
    }

    fn on_command(
        &mut self,
        command: crate::AppCommand<A>,
        _resolver: crate::AppResolver<A>,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) -> bool {
        let pos = state.render.cursor;
        let cell = &self.grid[pos];

        cell.on_command(pos, command, ctx, state.render, state.cell_state)
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
        state: &GridWidgetState<'a, C>,
    ) {
        let area = if state.render.options.draw_outer_borders {
            root.inner(Margin::new(1, 1))
        } else {
            root
        };

        tracing::trace!(
            "Render grid @ {} ({}x{})",
            state.render.cursor,
            self.grid.rows(),
            self.grid.cols()
        );
        tracing::trace!("Bordered area: {area:?}");
        tracing::trace!("Area: {area:?}");

        // Render the grid itself
        let opts = &state.render.options;

        let draw_inner = opts.draw_inner_borders;
        let cell_w = opts.cell_width;
        let cell_h = opts.cell_height;

        let offset = state.render.scroll_state.offset();
        let x_start = area.x + offset.x;
        let mut y = area.y + offset.y;

        let (rows, cols) = state.render.visible_ranges();

        for row in rows {
            let mut x = x_start;

            for col in cols.clone() {
                // Draw the value at the current position in the grid
                let pos = Position::new(row, col);
                let cell = &self.grid[pos];
                tracing::trace!("Drawing at {pos:?} [({x}, {y}) on screen]");

                let cell_area = Rect::new(x, y, cell_w, cell_h);
                cell.render_cell(pos, cell_area, buf, ctx, state.render, state.cell_state);

                // Draw a background for the whole cell
                x += cell_w;

                // Draw inner vertical border if defined
                let col = col as u16;

                if let Some(size) = opts.inner_borders
                    && (col + 1).is_multiple_of(size.width)
                {
                    if draw_inner {
                        for div_y in y..y + cell_h {
                            tracing::trace!("\tSetting │ at ({x}, {div_y}) on screen");
                            buf.set_stringn(x, div_y, "│", 1, opts.inner_border_style);
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

            if let Some(size) = opts.inner_borders
                && (row + 1).is_multiple_of(size.height)
            {
                let mut div_x = x_start;

                for col in 0..self.grid.cols() {
                    let col = col as u16;

                    if draw_inner {
                        let width = area.right().saturating_sub(div_x).min(cell_w) as usize;
                        let text = "─".repeat(width);

                        tracing::trace!(
                            "\tSetting ─ at ({div_x}..{}, {y}) on screen",
                            div_x + width as u16
                        );
                        buf.set_stringn(div_x, y, text, width, opts.inner_border_style);
                        tracing::trace!("\tDone");
                    }

                    div_x += cell_w;

                    if (col + 1).is_multiple_of(size.width) {
                        if draw_inner && y < area.bottom() {
                            tracing::trace!("\tSetting ┼ at ({div_x}, {y}) on screen");
                            buf.set_stringn(div_x, y, "┼", 1, opts.inner_border_style);
                            tracing::trace!("\tDone");
                        }

                        div_x += 1;
                    }
                }

                y += 1;
            }
        }

        // Render borders
        if opts.draw_outer_borders {
            render_borders(area, buf, state.render);
        }
    }
}
