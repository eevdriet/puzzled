mod actions;
mod options;
mod state;

pub use options::*;
pub use state::*;

use derive_more::{Debug, Deref, DerefMut};
use puzzled_core::{Grid, Position};
use ratatui::{
    layout::{Margin, Rect, Size},
    prelude::Buffer,
    style::{Color, Style},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::{CellRender, RenderSize, align_area, widgets::board::render_borders};

#[derive(Debug, Deref, DerefMut)]
pub struct GridWidget<'a, T>(pub &'a Grid<T>);

impl<'a, T> Clone for GridWidget<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for GridWidget<'a, T> {}

impl<'a, T> RenderSize for GridWidget<'a, T> {
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

impl<'a, T> StatefulWidget for GridWidget<'a, T>
where
    T: CellRender<GridRenderState>,
{
    type State = GridRenderState;

    fn render(self, parent: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Determine the actual area in which to render the grid
        let bordered_area = align_area(
            self.render_size(state),
            parent,
            state.options.h_align,
            state.options.v_align,
        );

        // Determine the grid area based on whether borders should be drawn
        let area = if state.options.draw_outer_borders {
            bordered_area.inner(Margin::new(1, 1))
        } else {
            bordered_area
        };
        state.viewport.area = area;

        // Render
        self.render_grid(area, buf, state);

        if state.options.draw_outer_borders {
            render_borders(bordered_area, buf, state);
        }
    }
}

impl<'a, T> GridWidget<'a, T>
where
    T: CellRender<GridRenderState>,
{
    pub fn render_grid(&self, area: Rect, buf: &mut Buffer, state: &GridRenderState) {
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

                let cell_widget = cell.render_cell(pos, state);
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
    }
}
