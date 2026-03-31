mod side;
mod state;

pub use side::*;
pub use state::*;

use std::{collections::HashMap, marker::PhantomData};

use crate::{
    AppContext, AppTypes, CellRender, EdgeRender, GridRenderState, GridWidget, GridWidgetState,
    Widget as AppWidget,
};
use derive_more::Debug;
use puzzled_core::{Grid, Side, SidedGrid, Sides};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect, Size},
};

#[derive(Debug)]
pub struct SidedGridWidgetState<'a, C, E> {
    pub grid: GridWidgetState<'a, C>,
    pub sides: &'a mut SidesRenderState,
    pub edge_state: E,
}

impl<'a, C, E> SidedGridWidgetState<'a, C, E> {
    fn render_state(&self) -> SidedGridRenderState {
        SidedGridRenderState {
            grid: self.grid.render.clone(),
            sides: *self.sides,
        }
    }
}

pub struct SidedGridRenderState {
    pub grid: GridRenderState,
    pub sides: SidesRenderState,
}

pub struct SidedGridWidget<'a, A: AppTypes, T, U, C, E> {
    pub grid: &'a Grid<T>,
    pub sides: &'a HashMap<Side, Vec<U>>,

    _marker: PhantomData<A>,
    _cell: PhantomData<C>,
    _edge: PhantomData<E>,
}

impl<'a, A: AppTypes, T, U, C, E> SidedGridWidget<'a, A, T, U, C, E> {
    pub fn new(grid: &'a Grid<T>, sides: &'a HashMap<Side, Vec<U>>) -> Self {
        Self {
            grid,
            sides,
            _marker: PhantomData,
            _cell: PhantomData,
            _edge: PhantomData,
        }
    }

    pub fn from_sided(sided: &'a SidedGrid<T, U>) -> Self {
        Self {
            grid: &sided.grid,
            sides: &sided.sides,
            _marker: PhantomData,
            _cell: PhantomData,
            _edge: PhantomData,
        }
    }
}

impl<'a, A, T, U, C, E> AppWidget<A> for SidedGridWidget<'a, A, T, U, C, E>
where
    A: AppTypes,
    T: CellRender<A, C>,
    U: EdgeRender<A, E>,
{
    type State = SidedGridWidgetState<'a, C, E>;

    fn render(
        &mut self,
        root: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        // Collect widgets and their render areas
        let area = self.render_area(root, ctx, state);

        let (mut grid_widget, [top_widget, right_widget, bottom_widget, left_widget]) =
            self.widgets();

        let (grid_area, [top_area, right_area, bottom_area, left_area]) =
            self.areas(area, ctx, state);

        // Render the grid
        grid_widget.render(grid_area, buf, ctx, &mut state.grid);

        // Render all defined sides
        if let Some(mut widget) = top_widget {
            widget.render(top_area, buf, ctx, state);
        }
        if let Some(mut widget) = right_widget {
            widget.render(right_area, buf, ctx, state);
        }
        if let Some(mut widget) = bottom_widget {
            widget.render(bottom_area, buf, ctx, state);
        }
        if let Some(mut widget) = left_widget {
            widget.render(left_area, buf, ctx, state);
        }
    }

    fn render_size(&self, area: Rect, ctx: &AppContext<A>, state: &mut Self::State) -> Size {
        let mut size = Size::ZERO;

        let (grid_size, [top_size, right_size, bottom_size, left_size]) =
            self.sizes(area, ctx, state);

        size.width += grid_size.width;
        size.width += left_size.width;
        size.width += right_size.width;

        size.height += grid_size.height;
        size.height += top_size.height;
        size.height += bottom_size.height;

        size
    }
}

type Widgets<'a, A, T, U, C, E> = (GridWidget<'a, A, T, C>, Sides<SideWidget<'a, A, U, C, E>>);

impl<'a, A, T, U, C, E> SidedGridWidget<'a, A, T, U, C, E>
where
    A: AppTypes,
    T: CellRender<A, C>,
    U: EdgeRender<A, E>,
{
    fn sizes(
        &self,
        area: Rect,
        ctx: &AppContext<A>,
        state: &mut SidedGridWidgetState<'a, C, E>,
    ) -> (Size, [Size; 4]) {
        // Grid area
        let (grid, sides) = self.widgets();

        let grid_size = grid.render_size(area, ctx, &mut state.grid);
        let side_sizes = sides.map(|side_widget| {
            side_widget
                .map(|widget| widget.render_size(area, ctx, state))
                .unwrap_or_default()
        });

        (grid_size, side_sizes)
    }

    fn widgets(&self) -> Widgets<'a, A, T, U, C, E> {
        let grid = GridWidget::<'a, A, T, C>::new(self.grid);

        let widget = |side: Side| {
            let edges = self.sides.get(&side)?;
            Some(SideWidget::<A, U, C, E>::new(side, edges))
        };

        (grid, Side::ALL.map(widget))
    }

    fn areas(
        &self,
        area: Rect,
        ctx: &AppContext<A>,
        state: &mut SidedGridWidgetState<'a, C, E>,
    ) -> (Rect, [Rect; 4]) {
        let (grid_size, [top_size, right_size, bottom_size, left_size]) =
            self.sizes(area, ctx, state);

        let max_top_height = state.sides.top.max_len.unwrap_or(u16::MAX);
        let max_right_width = state.sides.right.max_len.unwrap_or(u16::MAX);
        let max_bottom_height = state.sides.bottom.max_len.unwrap_or(u16::MAX);
        let max_left_width = state.sides.left.max_len.unwrap_or(u16::MAX);

        let [left_col, center_col, right_col] = Layout::horizontal([
            Constraint::Length(left_size.width.min(max_left_width)),
            Constraint::Max(grid_size.width),
            Constraint::Length(right_size.width.min(max_right_width)),
        ])
        .areas(area);

        let [top_area, grid_area, bottom_area] = Layout::vertical([
            Constraint::Length(top_size.height.min(max_top_height)),
            Constraint::Max(grid_size.height),
            Constraint::Length(bottom_size.height.min(max_bottom_height)),
        ])
        .areas(center_col);

        let left_area = Rect {
            x: left_col.x,
            y: grid_area.y,
            width: left_col.width,
            height: grid_area.height,
        };

        let right_area = Rect {
            x: right_col.x,
            y: grid_area.y,
            width: right_col.width,
            height: grid_area.height,
        };

        tracing::trace!("\tSizes");
        tracing::trace!("\t\tGrid: {grid_size:?}");
        tracing::trace!("\t\tTop: {top_size:?}");
        tracing::trace!("\t\tRight: {right_size:?}");
        tracing::trace!("\t\tBottom: {bottom_size:?}");
        tracing::trace!("\t\tLeft: {left_size:?}");

        tracing::trace!("\tAreas");
        tracing::trace!("\t\tGrid: {grid_area:?}");
        tracing::trace!("\t\tTop: {top_area:?}");
        tracing::trace!("\t\tRight: {right_area:?}");
        tracing::trace!("\t\tBottom: {bottom_area:?}");
        tracing::trace!("\t\tLeft: {left_area:?}");

        (grid_area, [top_area, right_area, bottom_area, left_area])
    }
}
