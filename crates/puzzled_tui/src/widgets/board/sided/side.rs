use crate::{
    AppTypes, LineRender, RenderSize, SidedGridRenderState, Widget as AppWidget, align_vertically,
};

use puzzled_core::Direction;
use ratatui::{
    layout::{HorizontalAlignment, VerticalAlignment},
    prelude::{Buffer, Rect, Size, Widget},
};

pub struct SideWidget<'a, U, E> {
    pub side: Direction,
    pub edges: &'a Vec<U>,
    pub edge_state: &'a E,
}

impl<'a, U, E> SideWidget<'a, U, E> {
    pub fn new(side: Direction, edges: &'a Vec<U>, edge_state: &'a E) -> Self {
        Self {
            side,
            edges,
            edge_state,
        }
    }

    fn render_vertical_side(&self, area: Rect, buf: &mut Buffer, state: &SidedGridRenderState)
    where
        U: LineRender<E>,
    {
        let opts = &state.grid.options;
        let margin = state.sides.get(self.side).margin.min(area.height);

        let (alignment, top, bottom) = match self.side {
            Direction::Up => (
                VerticalAlignment::Bottom,
                area.top(),
                area.bottom() - margin,
            ),
            Direction::Down => (VerticalAlignment::Top, area.top() + margin, area.bottom()),
            dir => unreachable!("{dir:?} should not be render as a horizontal side"),
        };

        let mut x = area.x;
        let cell_w = opts.cell_width;

        for (col, edge) in self.edges.iter().enumerate() {
            // Draw the value at the current row of the side
            let text = edge
                .render_col(col, self.edge_state)
                .alignment(opts.h_align);

            // Determine the area to render the value in
            let (text_y, text_h) = align_vertically(text.height() as u16, top, bottom, alignment);
            let text_area = Rect::new(x, text_y, cell_w, text_h);

            text.render(text_area, buf);

            // Advance y by cell height
            x += cell_w;

            // Skip over vertical divider
            let col = col as u16;

            if let Some(size) = opts.inner_borders
                && (col + 1).is_multiple_of(size.width)
            {
                x += 1;
            }
        }
    }

    fn render_horizontal_side(&self, area: Rect, buf: &mut Buffer, state: &SidedGridRenderState)
    where
        U: LineRender<E>,
    {
        let opts = &state.grid.options;
        let margin = state.sides.get(self.side).margin.min(area.width);

        let (alignment, left, right) = match self.side {
            Direction::Left => (
                HorizontalAlignment::Right,
                area.left(),
                area.right() - margin,
            ),
            Direction::Right => (
                HorizontalAlignment::Left,
                area.left() + margin,
                area.right(),
            ),
            dir => unreachable!("{dir:?} should not be render as a vertical side"),
        };

        let mut y = area.y;
        let cell_h = opts.cell_height;

        for (row, edge) in self.edges.iter().enumerate() {
            // Draw the value at the current row of the side
            let text = edge.render_row(row, self.edge_state).alignment(alignment);

            // Determine the area to render the value in
            let (text_y, text_h) =
                align_vertically(text.height() as u16, y, y + cell_h, opts.v_align);
            let text_area = Rect::new(left, text_y, right - left, text_h);

            text.render(text_area, buf);

            // Advance y by cell height
            y += cell_h;

            // Skip over horizontal divider
            let row = row as u16;

            if let Some(size) = opts.inner_borders
                && (row + 1).is_multiple_of(size.height)
            {
                y += 1;
            }
        }
    }
}

impl<'a, A, U, E> AppWidget<A> for SideWidget<'a, U, E>
where
    A: AppTypes,
    U: LineRender<E>,
{
    type State = SidedGridRenderState;

    fn render(&mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if self.side.is_vertical() {
            self.render_vertical_side(area, buf, state);
        } else {
            self.render_horizontal_side(area, buf, state);
        }
    }

    fn render_size(&self, area: Rect, state: &Self::State) -> Size {
        let mut size = Size::ZERO;

        // Determine the edge size based on its edges
        for edge_size in self.edges.iter().enumerate().map(|(idx, edge)| {
            let edge_text = if self.side.is_vertical() {
                edge.render_col(idx, self.edge_state)
            } else {
                edge.render_row(idx, self.edge_state)
            };
            edge_text.render_size(area, &())
        }) {
            if self.side.is_vertical() {
                size.height = edge_size.height.max(size.height);
                size.width += edge_size.width;
            } else {
                size.width = edge_size.width.max(size.width);
                size.height += edge_size.height;
            }
        }

        let margin = state.sides.get(self.side).margin;

        // Add side margin
        if self.side.is_vertical() {
            size.height += margin;
        } else {
            size.width += margin;
        }

        size
    }
}
