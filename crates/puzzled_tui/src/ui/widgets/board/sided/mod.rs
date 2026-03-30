mod side;
mod state;

pub use side::*;
pub use state::*;

use std::{collections::HashMap, marker::PhantomData};

use crate::{
    AppContext, AppTypes, CellRender, GridRenderState, GridWidget, LineRender, Widget as AppWidget,
};
use derive_more::Debug;
use puzzled_core::{Grid, Side, SidedGrid, Sides};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect, Size},
};

#[derive(Debug, Clone)]
pub struct SidedGridRenderState {
    pub grid: GridRenderState,
    pub sides: SidesRenderState,
}

pub struct SidedGridWidget<'a, A: AppTypes, T, U, C, E> {
    pub grid: &'a Grid<T>,
    pub sides: &'a HashMap<Side, Vec<U>>,

    pub cell_state: &'a C,
    pub edge_state: &'a E,
    _marker: PhantomData<A>,
}

impl<'a, A: AppTypes, T, U, C, E> SidedGridWidget<'a, A, T, U, C, E> {
    pub fn new(
        grid: &'a Grid<T>,
        sides: &'a HashMap<Side, Vec<U>>,
        cell_state: &'a C,
        edge_state: &'a E,
    ) -> Self {
        Self {
            grid,
            sides,
            cell_state,
            edge_state,
            _marker: PhantomData,
        }
    }

    pub fn from_sided(sided: &'a SidedGrid<T, U>, cell_state: &'a C, edge_state: &'a E) -> Self {
        Self {
            grid: &sided.grid,
            sides: &sided.sides,
            cell_state,
            edge_state,
            _marker: PhantomData,
        }
    }
}

impl<'a, A, T, U, C, E> AppWidget<A> for SidedGridWidget<'a, A, T, U, C, E>
where
    A: AppTypes,
    T: CellRender<A, C>,
    U: LineRender<A, E>,
{
    type State = SidedGridRenderState;

    fn render(
        &mut self,
        root: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        // Collect widgets and their render areas
        let area = AppWidget::<A>::render_area(self, root, ctx, state);
        tracing::info!("Root: {root:?}");
        tracing::info!("Area: {area:?}");

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

    fn render_size(&self, area: Rect, ctx: &AppContext<A>, state: &Self::State) -> Size {
        // Grid
        let grid_state = &state.grid;
        let grid = GridWidget::<'a, A, T, C>::new(self.grid, self.cell_state);

        let mut size = AppWidget::<A>::render_size(&grid, area, ctx, grid_state);

        // Sides
        for (dir, edges) in self.sides.iter() {
            let side = SideWidget::<A, U, E>::new(*dir, edges, self.edge_state);
            let side_size = side.render_size(area, ctx, state);

            size.width += side_size.width;
            size.height += side_size.height;
        }

        size
    }

    // fn render_area(&self, area: Rect, ctx: &AppContext<A>, state: &Self::State) -> Rect {
    //     let size = self.render_size(area, ctx, state);
    //     top_left_area(area, size)
    // }
}

type Widgets<'a, A, T, U, C, E> = (GridWidget<'a, A, T, C>, Sides<SideWidget<'a, A, U, E>>);

impl<'a, A, T, U, C, E> SidedGridWidget<'a, A, T, U, C, E>
where
    A: AppTypes,
    T: CellRender<A, C>,
    U: LineRender<A, E>,
{
    fn sizes(
        &self,
        area: Rect,
        ctx: &AppContext<A>,
        state: &SidedGridRenderState,
    ) -> (Size, [Size; 4]) {
        // Grid area
        let (grid, sides) = self.widgets();

        let grid_size = grid.render_size(area, ctx, &state.grid);
        let side_sizes = sides.map(|side_widget| {
            side_widget
                .map(|widget| widget.render_size(area, ctx, state))
                .unwrap_or_default()
        });

        (grid_size, side_sizes)
    }

    fn widgets(&self) -> Widgets<'a, A, T, U, C, E> {
        let grid = GridWidget::<'a, A, T, C>::new(self.grid, self.cell_state);

        let widget = |side: Side| {
            let edges = self.sides.get(&side)?;
            Some(SideWidget::<A, U, E>::new(side, edges, self.edge_state))
        };

        (grid, Side::ALL.map(widget))
    }

    fn areas(
        &self,
        area: Rect,
        ctx: &AppContext<A>,
        state: &SidedGridRenderState,
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

        tracing::info!("\tSizes");
        tracing::info!("\t\tGrid: {grid_size:?}");
        tracing::info!("\t\tTop: {top_size:?}");
        tracing::info!("\t\tRight: {right_size:?}");
        tracing::info!("\t\tBottom: {bottom_size:?}");
        tracing::info!("\t\tLeft: {left_size:?}");

        // let top_area = Rect {
        //     x: area.left() + left_size.width,
        //     y: area.top(),
        //     width: top_size.width,
        //     height: top_size.height,
        // };
        //
        // // - Right
        // let right_area = Rect {
        //     x: area.left() + left_size.width + grid_size.width,
        //     y: area.top() + top_size.height,
        //     width: right_size.width,
        //     height: right_size.height,
        // };
        //
        // // - Bottom
        // let bottom_area = Rect {
        //     x: area.left() + left_size.width,
        //     y: area.top() + top_size.height + grid_size.height,
        //     width: bottom_size.width,
        //     height: bottom_size.height,
        // };
        //
        // // - Left
        // let left_area = Rect {
        //     x: area.left(),
        //     y: area.top() + top_size.height,
        //     width: left_size.width,
        //     height: left_size.height,
        // };

        // Compute the grid area from the sides
        // let [_, grid_area, _] = Layout::horizontal(vec![
        //     Constraint::Length(left_area.width),
        //     Constraint::Min(0),
        //     Constraint::Length(right_area.width),
        // ])
        // .areas(area);
        //
        // let [_, grid_area, _] = Layout::vertical(vec![
        //     Constraint::Length(top_area.height),
        //     Constraint::Min(0),
        //     Constraint::Length(bottom_area.height),
        // ])
        // .areas(grid_area);
        // let grid_area = Rect {
        //     x: area.left() + left_size.width,
        //     y: area.top() + top_size.height,
        //     width: grid_size.width,
        //     height: grid_size.height,
        // };

        tracing::info!("\tAreas");
        tracing::info!("\t\tGrid: {grid_area:?}");
        tracing::info!("\t\tTop: {top_area:?}");
        tracing::info!("\t\tRight: {right_area:?}");
        tracing::info!("\t\tBottom: {bottom_area:?}");
        tracing::info!("\t\tLeft: {left_area:?}");

        (grid_area, [top_area, right_area, bottom_area, left_area])
    }
}
