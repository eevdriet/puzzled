mod actions;
mod options;
mod state;

pub use options::*;
pub use state::*;

use puzzled_core::{Grid, Position};
use ratatui::{
    layout::{Margin, Rect, Size},
    prelude::Buffer,
    widgets::Widget,
};

use crate::{CellRender, RenderSize, widgets::board::render_borders};

impl<T> RenderSize for Grid<T> {
    type State = GridRenderState;

    fn render_size(&self, state: &Self::State) -> Size {
        let cols = state.viewport.cols().min(self.cols()) as u16;
        let rows = state.viewport.rows().min(self.rows()) as u16;

        let mut width = cols * state.options.cell_width;
        let mut height = rows * state.options.cell_height;

        if let Some(inner) = state.options.inner {
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

pub trait RenderGrid<S> {
    fn render(&self, area: Rect, buf: &mut Buffer, render_state: &GridRenderState, cell_state: &S);
}

impl<T, S> RenderGrid<S> for Grid<T>
where
    T: CellRender<S>,
{
    fn render(
        &self,
        bordered_area: Rect,
        buf: &mut Buffer,
        state: &GridRenderState,
        cell_state: &S,
    ) {
        let area = if state.options.draw_outer_borders {
            bordered_area.inner(Margin::new(1, 1))
        } else {
            bordered_area
        };

        // Render the grid itself
        let vp = &state.viewport;
        let opts = &state.options;

        let draw_inner = opts.draw_inner_borders;
        let cell_w = opts.cell_width;
        let cell_h = opts.cell_height;

        let x_start = area.x;
        let mut y = area.y;

        for row in vp.row_start..vp.row_end {
            let mut x = x_start;

            for col in vp.col_start..vp.col_end {
                // Draw the value at the current position in the grid
                let pos = Position::new(row, col);
                let cell = &self[pos];
                let cell_area = Rect::new(x, y, cell_w, cell_h);

                let cell_widget = cell.render_cell(pos, cell_state);
                cell_widget.render(cell_area, buf);

                // Draw a background for the whole cell

                x += cell_w;

                // Draw inner vertical border if defined
                let col = col as u16;

                if let Some(size) = state.options.inner
                    && (col + 1).is_multiple_of(size.width)
                {
                    if draw_inner {
                        for div_y in y..y + cell_h {
                            buf.set_stringn(x, div_y, "│", 1, state.options.inner_border_style);
                        }
                    }

                    x += 1;
                }
            }

            // Advance y by cell height
            y += cell_h;

            // Draw horizontal divider
            let row = row as u16;

            if let Some(size) = state.options.inner
                && (row + 1).is_multiple_of(size.height)
            {
                let mut div_x = x_start;

                for col in vp.col_start..vp.col_end {
                    let col = col as u16;

                    if draw_inner {
                        let text = "─".repeat(cell_w as usize);
                        buf.set_stringn(
                            div_x,
                            y,
                            text,
                            cell_w as usize,
                            state.options.inner_border_style,
                        );
                    }

                    div_x += cell_w;

                    if (col + 1).is_multiple_of(size.width) {
                        if draw_inner {
                            buf.set_stringn(div_x, y, "┼", 1, state.options.inner_border_style);
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
