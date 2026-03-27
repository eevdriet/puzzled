mod side;
mod state;

pub use side::*;
pub use state::*;

use std::collections::HashMap;

use crate::{
    AppContext, AppTypes, CellRender, GridRenderState, GridWidget, LineRender, Widget as AppWidget,
};
use derive_more::Debug;
use puzzled_core::{Direction, Grid, SidedGrid};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect, Size},
};

#[derive(Debug, Clone)]
pub struct SidedGridRenderState {
    pub grid: GridRenderState,
    pub sides: SidesRenderState,
}

pub struct SidedGridWidget<'a, T, U, C, E> {
    pub grid: &'a Grid<T>,
    pub sides: &'a HashMap<Direction, Vec<U>>,

    pub cell_state: &'a C,
    pub edge_state: &'a E,
}

impl<'a, T, U, C, E> SidedGridWidget<'a, T, U, C, E> {
    pub fn new(
        grid: &'a Grid<T>,
        sides: &'a HashMap<Direction, Vec<U>>,
        cell_state: &'a C,
        edge_state: &'a E,
    ) -> Self {
        Self {
            grid,
            sides,
            cell_state,
            edge_state,
        }
    }

    pub fn from_sided(sided: &'a SidedGrid<T, U>, cell_state: &'a C, edge_state: &'a E) -> Self {
        Self {
            grid: &sided.grid,
            sides: &sided.sides,
            cell_state,
            edge_state,
        }
    }
}

impl<'a, A, T, U, C, E> AppWidget<A> for SidedGridWidget<'a, T, U, C, E>
where
    A: AppTypes,
    T: CellRender<C>,
    U: LineRender<E>,
{
    type State = SidedGridRenderState;

    fn render(
        &mut self,
        root: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        let area = AppWidget::<A>::render_area(self, root, ctx, state);

        // Collect side widgets and sizes
        let mut top_size = Size::ZERO;
        let mut right_size = Size::ZERO;
        let mut bottom_size = Size::ZERO;
        let mut left_size = Size::ZERO;

        let mut top_widget = None;
        let mut right_widget = None;
        let mut bottom_widget = None;
        let mut left_widget = None;

        let get_widget_and_size =
            |dir: Direction, widget: &mut Option<SideWidget<'a, U, E>>, size: &mut Size| {
                if let Some(side) = self.sides.get(&dir) {
                    let side_widget = SideWidget::new(dir, side, self.edge_state);
                    let side_size = AppWidget::<A>::render_size(&side_widget, area, ctx, state);

                    *size = side_size;
                    *widget = Some(side_widget);
                }
            };

        get_widget_and_size(Direction::Up, &mut top_widget, &mut top_size);
        get_widget_and_size(Direction::Right, &mut right_widget, &mut right_size);
        get_widget_and_size(Direction::Down, &mut bottom_widget, &mut bottom_size);
        get_widget_and_size(Direction::Left, &mut left_widget, &mut left_size);

        // Then render the sides in the correct area
        // - Top
        if let Some(mut top) = top_widget {
            let top_area = Rect {
                x: area.left() + left_size.width,
                y: area.top(),
                width: top_size.width,
                height: top_size.height,
            };
            AppWidget::<A>::render(&mut top, top_area, buf, ctx, state);
        }

        // - Right
        if let Some(mut right) = right_widget {
            let right_area = Rect {
                x: area.right().saturating_sub(right_size.width),
                y: area.top() + top_size.height,
                width: right_size.width,
                height: right_size.height,
            };
            AppWidget::<A>::render(&mut right, right_area, buf, ctx, state);
        }

        // - Bottom
        if let Some(mut bottom) = bottom_widget {
            let bottom_area = Rect {
                x: area.left() + left_size.width,
                y: area.bottom().saturating_sub(bottom_size.height),
                width: bottom_size.width,
                height: bottom_size.height,
            };
            AppWidget::<A>::render(&mut bottom, bottom_area, buf, ctx, state);
        }

        // - Left
        if let Some(mut left) = left_widget {
            let left_area = Rect {
                x: area.left(),
                y: area.top() + top_size.height,
                width: left_size.width,
                height: left_size.height,
            };
            AppWidget::<A>::render(&mut left, left_area, buf, ctx, state);
        }

        // Compute the grid area from the sides
        let [_, grid_area, _] = Layout::horizontal(vec![
            Constraint::Length(left_size.width),
            Constraint::Min(0),
            Constraint::Length(right_size.width),
        ])
        .areas(area);

        let [_, grid_area, _] = Layout::vertical(vec![
            Constraint::Length(top_size.height),
            Constraint::Min(0),
            Constraint::Length(bottom_size.height),
        ])
        .areas(grid_area);

        // Render the grid
        let mut grid_widget = GridWidget::new(self.grid, self.cell_state);
        AppWidget::<A>::render(&mut grid_widget, grid_area, buf, ctx, &mut state.grid);
    }

    fn render_size(&self, area: Rect, ctx: &AppContext<A>, state: &Self::State) -> Size {
        // Grid
        let grid_state = &state.grid;
        let grid = GridWidget::<'a, T, C>::new(self.grid, self.cell_state);

        let mut size = AppWidget::<A>::render_size(&grid, area, ctx, grid_state);

        // Sides
        for (dir, edges) in self.sides.iter() {
            let side = SideWidget::new(*dir, edges, self.edge_state);
            let side_size = AppWidget::<A>::render_size(&side, area, ctx, state);

            size.width += side_size.width;
            size.height += side_size.height;
        }

        size
    }
}
