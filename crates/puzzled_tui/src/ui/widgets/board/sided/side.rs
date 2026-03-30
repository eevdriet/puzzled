use std::marker::PhantomData;

use crate::{
    AppContext, AppTypes, LineRender, RenderSize, ScrollWidget, SidedGridRenderState,
    Widget as AppWidget, align_vertically,
};

use puzzled_core::Side;
use ratatui::{
    layout::{HorizontalAlignment, Position, VerticalAlignment},
    prelude::{Buffer, Rect, Size, Widget},
};
use tui_scrollview::ScrollbarVisibility;

pub struct SideWidget<'a, A: AppTypes, U, E> {
    pub side: Side,
    pub edges: &'a Vec<U>,
    pub edge_state: &'a E,

    _marker: PhantomData<A>,
}

impl<'a, A: AppTypes, U, E> SideWidget<'a, A, U, E> {
    pub fn new(side: Side, edges: &'a Vec<U>, edge_state: &'a E) -> Self {
        Self {
            side,
            edges,
            edge_state,
            _marker: PhantomData,
        }
    }

    fn render_cols(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &SidedGridRenderState,
        ctx: &AppContext<A>,
    ) where
        U: LineRender<A, E>,
    {
        let opts = &state.grid.options;
        let margin = state.sides.get(self.side).margin.min(area.height);

        let (alignment, top, bottom) = match self.side {
            Side::Top => (
                VerticalAlignment::Bottom,
                area.top(),
                area.bottom() - margin,
            ),
            Side::Bottom => (VerticalAlignment::Top, area.top() + margin, area.bottom()),
            side => unreachable!("{side:?} should not render as a horizontal side"),
        };

        let mut x = area.x;
        let cell_w = opts.cell_width;

        for (col, edge) in self.edges.iter().enumerate() {
            // Draw the value at the current row of the side
            let text = edge
                .render_col(col, self.edge_state, ctx)
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

    fn render_rows(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &SidedGridRenderState,
        ctx: &AppContext<A>,
    ) where
        U: LineRender<A, E>,
    {
        let opts = &state.grid.options;
        let margin = state.sides.get(self.side).margin.min(area.width);

        let (alignment, left, right) = match self.side {
            Side::Left => (
                HorizontalAlignment::Right,
                area.left(),
                area.right() - margin,
            ),
            Side::Right => (
                HorizontalAlignment::Left,
                area.left() + margin,
                area.right(),
            ),
            side => unreachable!("{side:?} should not render as a vertical side"),
        };

        let mut y = area.y;
        let cell_h = opts.cell_height;

        for (row, edge) in self.edges.iter().enumerate() {
            // Draw the value at the current row of the side
            let text = edge
                .render_row(row, self.edge_state, ctx)
                .alignment(alignment);

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

impl<'a, A, U, E> AppWidget<A> for SideWidget<'a, A, U, E>
where
    A: AppTypes,
    U: LineRender<A, E>,
{
    type State = SidedGridRenderState;

    fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        let size = self.render_size(area, ctx, state);
        let scroll_area = Rect::from(size);
        let mut scroll_state = state.grid.scroll_state;
        let offset = scroll_state.offset();

        let mut scroll_view =
            ScrollWidget::new(size).scrollbars_visibility(ScrollbarVisibility::Never);

        if self.side.is_vertical() {
            scroll_state.set_offset(Position { y: 0, ..offset });
            self.render_cols(scroll_area, scroll_view.buf_mut(), state, ctx);
        } else {
            scroll_state.set_offset(Position { x: 0, ..offset });
            self.render_rows(scroll_area, scroll_view.buf_mut(), state, ctx);
        }

        scroll_view.render(area, buf, ctx, &mut scroll_state);
    }

    fn render_size(&self, area: Rect, ctx: &AppContext<A>, state: &Self::State) -> Size {
        let mut len = 0;
        let opts = &state.grid.options;

        let cell_w = opts.cell_width;
        let cell_h = opts.cell_height;

        // Determine the maximum edge length
        let mut max_edge_len = |idx: usize, edge: &U| {
            // Render the edge and compute its size
            let edge_text = if self.side.is_vertical() {
                edge.render_col(idx, self.edge_state, ctx)
            } else {
                edge.render_row(idx, self.edge_state, ctx)
            };
            let edge_size = edge_text.render_size(area, &());

            // Add the edge size to the total size
            if self.side.is_vertical() {
                len = edge_size.height.max(len);
            } else {
                len = edge_size.width.max(len);
            }
        };

        for (idx, edge) in self.edges.iter().enumerate() {
            max_edge_len(idx, edge);
        }

        // Construct the side size from the cell dimensions
        let edge_count = self.edges.len() as u16;
        let mut width = edge_count * cell_w;
        let mut height = edge_count * cell_h;

        if let Some(inner) = opts.inner_borders {
            width += edge_count / inner.width;
            height += edge_count / inner.height;
        }

        let mut size = if self.side.is_vertical() {
            Size { width, height: len }
        } else {
            Size { height, width: len }
        };

        // Add side margin
        let margin = state.sides.get(self.side).margin;

        if self.side.is_vertical() {
            size.height += margin;
        } else {
            size.width += margin;
        }

        // Apply the maximum allowed width and height
        let side_state = state.sides.get(self.side);

        if let Some(max_len) = side_state.max_len {
            if self.side.is_vertical() {
                size.height = size.height.min(max_len);
            } else {
                size.width = size.width.min(max_len);
            }
        }

        size
    }
}
