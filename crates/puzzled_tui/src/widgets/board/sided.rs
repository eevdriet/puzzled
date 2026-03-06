use crate::{
    CellRender, GridOptions, GridRenderState, GridWidget, LineRender, RenderSize, Viewport,
    align_area, align_vertically,
};
use derive_more::{Debug, Deref, DerefMut};
use puzzled_core::{Position, SidedGrid};
use ratatui::{
    layout::{HorizontalAlignment, Margin, VerticalAlignment},
    prelude::{Buffer, Rect},
    widgets::{StatefulWidget, Widget},
};

#[derive(Debug, Clone, Copy)]
pub struct SideOptions {
    pub top_height: u16,
    pub bottom_height: u16,

    pub left_width: u16,
    pub right_width: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct SidedGridState {
    pub grid_options: GridOptions,
    pub side_options: SideOptions,

    pub viewport: Viewport,
    pub cursor: Position,
}

impl From<SidedGridState> for GridRenderState {
    fn from(state: SidedGridState) -> Self {
        Self {
            options: state.grid_options,
            viewport: state.viewport,
            cursor: state.cursor,
        }
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct SidedGridWidget<'a, T, U>(pub &'a SidedGrid<T, U>);

impl<'a, T, U> RenderSize for SidedGridWidget<'a, T, U> {
    type State = SidedGridState;

    fn render_size(&self, state: &SidedGridState) -> ratatui::prelude::Size {
        // First determine the size of the inner grid
        let grid = GridWidget(&self.grid);
        let grid_state = GridRenderState::from(*state);

        let mut size = grid.render_size(&grid_state);

        // Then add on the side dimensionality
        let opts = &state.side_options;

        // Vertical sides
        if self.top.is_some() {
            size.height += opts.top_height;
        }
        if self.bottom.is_some() {
            size.height += opts.bottom_height;
        }

        // Horizontal sided
        if self.left.is_some() {
            size.width += opts.left_width;
        }
        if self.right.is_some() {
            size.width += opts.right_width;
        }

        size
    }
}

impl<'a, T, U> StatefulWidget for SidedGridWidget<'a, T, U>
where
    T: CellRender<GridRenderState>,
    U: LineRender<SidedGridState>,
{
    type State = SidedGridState;

    fn render(self, parent: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Determine the actual areas in which to render the grid + sides
        let area = align_area(
            self.render_size(state),
            parent,
            state.grid_options.h_align,
            state.grid_options.v_align,
        );
        let grid_area = area.inner(Margin::new(
            state.side_options.left_width,
            state.side_options.top_height,
        ));

        // Render the grid
        let grid = GridWidget(&self.grid);
        let mut grid_state = GridRenderState::from(*state);

        grid.render(grid_area, buf, &mut grid_state);

        // Render the sides
        self.render_sides(area, buf, state);
    }
}

impl<'a, T, U> SidedGridWidget<'a, T, U>
where
    U: LineRender<SidedGridState>,
{
    pub fn render_sides(&self, area: Rect, buf: &mut Buffer, state: &SidedGridState) {
        let border = state.grid_options.draw_outer_borders as u16;
        let opts = &state.side_options;
        let width = area.width - opts.left_width - opts.right_width - 2 * border;
        let height = area.height - opts.top_height - opts.bottom_height - 2 * border;

        if let Some(top) = self.top.as_ref() {
            let area = Rect {
                x: area.left() + opts.left_width + border,
                y: area.top(),
                width,
                height: opts.top_height,
            };

            self.render_horizontal_side(top, VerticalAlignment::Bottom, area, buf, state);
        }

        if let Some(right) = self.right.as_ref() {
            let area = Rect {
                x: area.right() - opts.right_width,
                y: area.top() + opts.top_height + border,
                width: opts.right_width,
                height,
            };

            self.render_vertical_side(right, HorizontalAlignment::Left, area, buf, state);
        }

        if let Some(bottom) = self.bottom.as_ref() {
            let area = Rect {
                x: area.left() + opts.left_width + border,
                y: area.bottom() - opts.bottom_height,
                width,
                height: opts.bottom_height,
            };

            self.render_horizontal_side(bottom, VerticalAlignment::Top, area, buf, state);
        }

        if let Some(left) = self.left.as_ref() {
            let area = Rect {
                x: area.left(),
                y: area.top() + opts.top_height + border,
                width: opts.left_width,
                height,
            };

            self.render_vertical_side(left, HorizontalAlignment::Right, area, buf, state);
        }
    }

    pub fn render_horizontal_side(
        &self,
        values: &[U],
        side: VerticalAlignment,
        area: Rect,
        buf: &mut Buffer,
        state: &SidedGridState,
    ) {
        let vp = &state.viewport;
        let opts = &state.grid_options;

        let mut x = area.x;
        let cell_w = opts.cell_width;

        for (col, value) in values
            .iter()
            .enumerate()
            .take(vp.col_end)
            .skip(vp.col_start)
        {
            // Draw the value at the current row of the side
            let text = value.render_col(col, state).alignment(opts.h_align);

            // Determine the area to render the value in
            let (text_y, text_h) =
                align_vertically(text.height() as u16, area.top(), area.bottom(), side);
            let text_area = Rect::new(x, text_y, cell_w, text_h);

            text.render(text_area, buf);

            // Advance y by cell height
            x += cell_w;

            // Skip over vertical divider
            let col = col as u16;

            if let Some(size) = opts.inner
                && (col + 1).is_multiple_of(size.width)
            {
                x += 1;
            }
        }
    }

    pub fn render_vertical_side(
        &self,
        values: &[U],
        side: HorizontalAlignment,
        area: Rect,
        buf: &mut Buffer,
        state: &SidedGridState,
    ) {
        let vp = &state.viewport;
        let opts = &state.grid_options;

        let x = area.x;
        let mut y = area.y;
        let cell_h = opts.cell_height;

        for (row, value) in values
            .iter()
            .enumerate()
            .take(vp.row_end)
            .skip(vp.row_start)
        {
            // Draw the value at the current row of the side
            let text = value.render_row(row, state).alignment(side);

            // Determine the area to render the value in
            let (text_y, text_h) =
                align_vertically(text.height() as u16, y, y + cell_h, opts.v_align);
            let text_area = Rect::new(x, text_y, area.width, text_h);

            text.render(text_area, buf);

            // Advance y by cell height
            y += cell_h;

            // Skip over horizontal divider
            let row = row as u16;

            if let Some(size) = opts.inner
                && (row + 1).is_multiple_of(size.height)
            {
                y += 1;
            }
        }
    }
}
