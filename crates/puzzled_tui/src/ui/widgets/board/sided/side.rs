use std::marker::PhantomData;

use crate::{
    AppContext, AppTypes, EdgeRender, ScrollWidget, SideRenderState, SidedGridWidgetState,
    Widget as AppWidget, align_horizontally, align_vertically,
};

use puzzled_core::Side;
use ratatui::{
    layout::{HorizontalAlignment, Position, VerticalAlignment},
    prelude::{Buffer, Rect, Size},
};
use tui_scrollview::ScrollbarVisibility;

pub struct SideWidget<'a, A: AppTypes, U, C, E> {
    pub side: Side,
    pub edges: &'a Vec<U>,

    _app: PhantomData<A>,
    _cell: PhantomData<C>,
    _edge: PhantomData<E>,
}

impl<'a, A: AppTypes, U, C, E> SideWidget<'a, A, U, C, E> {
    pub fn new(side: Side, edges: &'a Vec<U>) -> Self {
        Self {
            side,
            edges,
            _app: PhantomData,
            _cell: PhantomData,
            _edge: PhantomData,
        }
    }

    fn render_cols(
        &self,
        area: Rect,
        buf: &mut Buffer,
        ctx: &AppContext<A>,
        state: &SidedGridWidgetState<C, E>,
    ) where
        U: EdgeRender<A, E>,
    {
        let render = &state.grid.render;
        let side_state = state.sides.get(self.side);
        let render_state = state.render_state();
        let margin = side_state.margin.min(area.height);

        let (alignment, top, bottom) = match self.side {
            Side::Top => (
                VerticalAlignment::Bottom,
                area.top(),
                area.bottom() - margin,
            ),
            Side::Bottom => (VerticalAlignment::Top, area.top() + margin, area.bottom()),
            side => unreachable!("{side:?} should not render as a horizontal side"),
        };

        let offset = render.scroll_state.offset();
        let cell_w = render.options.cell_width;
        let (_, cols) = state.grid.render.visible_ranges();

        let mut x = area.x + offset.x;

        for (col, edge) in self.edges.iter().enumerate().skip(cols.start) {
            tracing::trace!("Col {col}");

            // Determine the area to render the value in
            let height = edge.height(area, ctx, &render_state, state.edge_state);
            let (text_y, text_h) = align_vertically(height, top, bottom, alignment);
            let text_area = Rect::new(x, text_y, cell_w, text_h);

            tracing::trace!("\tEdge height: {height}");
            tracing::trace!("\tText x/y: ({x}, {text_y})");
            tracing::trace!("\tText height: {text_h}");
            tracing::trace!("\tText area: {text_area:?}");

            // Draw the value at the current column of the side
            edge.render_col(col, text_area, buf, ctx, &render_state, state.edge_state);

            // Advance y by cell height
            x += cell_w;

            // Skip over vertical divider
            let col = col as u16;

            if let Some(size) = render.options.inner_borders
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
        ctx: &AppContext<A>,
        state: &SidedGridWidgetState<C, E>,
    ) where
        U: EdgeRender<A, E>,
    {
        let render = &state.grid.render;
        let side_state = state.sides.get(self.side);
        let render_state = state.render_state();
        let margin = side_state.margin.min(area.width);

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

        let _offset = render.scroll_state.offset();
        let cell_h = render.options.cell_height;
        let (_, rows) = state.grid.render.visible_ranges();

        let mut y = area.y;

        for (row, edge) in self.edges.iter().enumerate().skip(rows.start) {
            tracing::trace!("Row {row}");
            // Determine the area to render the value in
            let width = edge.width(area, ctx, &render_state, state.edge_state);
            let (text_x, text_w) = align_horizontally(width, left, right, alignment);
            let text_area = Rect::new(text_x, y, text_w, cell_h);

            tracing::trace!("\tEdge width: {width}");
            tracing::trace!("\tText x/y: ({text_x}, {y})");
            tracing::trace!("\tText width: {text_w}");
            tracing::trace!("\tText area: {text_area:?}");

            // Draw the value at the current row of the side
            edge.render_row(row, text_area, buf, ctx, &render_state, state.edge_state);

            // Advance y by cell height
            y += cell_h;

            // Skip over horizontal divider
            let row = row as u16;

            if let Some(size) = render.options.inner_borders
                && (row + 1).is_multiple_of(size.height)
            {
                y += 1;
            }
        }
    }
}

impl<'a, A, U, C, E> AppWidget<A> for SideWidget<'a, A, U, C, E>
where
    A: AppTypes,
    U: EdgeRender<A, E>,
    C: 'a,
    E: 'a,
    U: 'a,
{
    type State = SidedGridWidgetState<'a, C, E>;

    fn render(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        let side_state = state.sides.get_mut(self.side);
        side_state.viewport = area;

        let size = self.render_size(area, ctx, state);
        let scroll_area = Rect::from(size);
        let mut scroll_state = state.grid.render.scroll_state;
        let offset = scroll_state.offset();

        let mut scroll_view =
            ScrollWidget::new(size).scrollbars_visibility(ScrollbarVisibility::Never);

        if self.side.is_vertical() {
            scroll_state.set_offset(Position { y: 0, ..offset });
            self.render_cols(area, buf, ctx, state);
        } else {
            scroll_state.set_offset(Position { x: 0, ..offset });
            self.render_rows(area, buf, ctx, state);
        }

        // scroll_view.render(area, buf, ctx, &mut scroll_state);
    }

    fn render_size(&self, area: Rect, ctx: &AppContext<A>, state: &mut Self::State) -> Size {
        let mut len = 0;
        let render_state = state.render_state();
        let opts = &state.grid.render.options;

        let cell_w = opts.cell_width;
        let cell_h = opts.cell_height;

        // Determine the maximum edge length
        for edge in self.edges.iter() {
            if self.side.is_vertical() {
                let height = edge.height(area, ctx, &render_state, state.edge_state);
                len = height.max(len);
            } else {
                let width = edge.width(area, ctx, &render_state, state.edge_state);
                len = width.max(len);
            }
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
        let SideRenderState {
            margin, max_len, ..
        } = state.sides.get(self.side);

        if self.side.is_vertical() {
            size.height += margin;
        } else {
            size.width += margin;
        }

        // Apply the maximum allowed width and height
        if let Some(max_len) = max_len {
            if self.side.is_vertical() {
                size.height = size.height.min(*max_len);
            } else {
                size.width = size.width.min(*max_len);
            }
        }

        size
    }
}
