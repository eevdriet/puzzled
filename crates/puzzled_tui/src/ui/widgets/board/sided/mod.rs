mod builder;
mod render;
mod side;
mod state;

pub use render::*;
pub use side::*;
pub use state::*;

use std::marker::PhantomData;

use crate::{AppContext, AppTypes, CellRender, EdgeRender, GridWidget, Widget as AppWidget};
use puzzled_core::{Grid, Side, SidedGrid};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect, Size},
};

pub struct SidedGridWidget<'a, A: AppTypes, CT, TT, RT, BT, LT, C, E> {
    pub grid: &'a Grid<CT>,
    pub top: Option<&'a Vec<TT>>,
    pub right: Option<&'a Vec<RT>>,
    pub bottom: Option<&'a Vec<BT>>,
    pub left: Option<&'a Vec<LT>>,

    _marker: PhantomData<A>,
    _cell: PhantomData<C>,
    _edge: PhantomData<E>,
}

impl<'a, A: AppTypes, CT, C, E> SidedGridWidget<'a, A, CT, (), (), (), (), C, E> {
    pub fn from_grid(grid: &'a Grid<CT>) -> SidedGridWidget<'a, A, CT, (), (), (), (), C, E> {
        SidedGridWidget {
            grid,
            top: None,
            right: None,
            bottom: None,
            left: None,
            _marker: PhantomData,
            _cell: PhantomData,
            _edge: PhantomData,
        }
    }
}

impl<'a, A: AppTypes, CT, TT, RT, BT, LT, C, E> SidedGridWidget<'a, A, CT, TT, RT, BT, LT, C, E> {
    pub fn from_sided(sided: &'a SidedGrid<CT, TT, RT, BT, LT>) -> Self {
        Self {
            grid: &sided.grid,
            top: sided.top.as_ref(),
            right: sided.right.as_ref(),
            bottom: sided.bottom.as_ref(),
            left: sided.left.as_ref(),
            _marker: PhantomData,
            _cell: PhantomData,
            _edge: PhantomData,
        }
    }
}

impl<'a, A, CT, TT, RT, BT, LT, C, E> AppWidget<A>
    for SidedGridWidget<'a, A, CT, TT, RT, BT, LT, C, E>
where
    A: AppTypes,
    CT: CellRender<A, C>,
    TT: EdgeRender<A, E>,
    RT: EdgeRender<A, E>,
    BT: EdgeRender<A, E>,
    LT: EdgeRender<A, E>,
    C: 'a,
    E: 'a,
{
    type State = SidedGridWidgetState<'a, C, E>;

    fn render(
        &mut self,
        root: Rect,
        buf: &mut Buffer,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) {
        let area = self.render_area(root, ctx, state);

        // Widgets
        let mut grid_widget = GridWidget::new(self.grid);
        let top_widget = self.side_widget(Side::Top, self.top);
        let right_widget = self.side_widget(Side::Right, self.right);
        let bottom_widget = self.side_widget(Side::Bottom, self.bottom);
        let left_widget = self.side_widget(Side::Left, self.left);

        // Sizes
        let top_size = self.side_size(Side::Top, self.top, area, ctx, state);
        let right_size = self.side_size(Side::Right, self.right, area, ctx, state);
        let bottom_size = self.side_size(Side::Bottom, self.bottom, area, ctx, state);
        let left_size = self.side_size(Side::Left, self.left, area, ctx, state);

        let max_top_height = state.render.sides.top.max_len.unwrap_or(u16::MAX);
        let max_right_width = state.render.sides.right.max_len.unwrap_or(u16::MAX);
        let max_bottom_height = state.render.sides.bottom.max_len.unwrap_or(u16::MAX);
        let max_left_width = state.render.sides.left.max_len.unwrap_or(u16::MAX);

        let grid_size = grid_widget.render_size(area, ctx, &mut state.grid_state());

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
        tracing::trace!("\t\tRoot: {root:?}");
        tracing::trace!("\t\tArea: {area:?}");
        tracing::trace!("\t\tGrid: {grid_area:?}");
        tracing::trace!("\t\tTop: {top_area:?}");
        tracing::trace!("\t\tRight: {right_area:?}");
        tracing::trace!("\t\tBottom: {bottom_area:?}");
        tracing::trace!("\t\tLeft: {left_area:?}");

        // Render the grid
        tracing::trace!("\tRendering");
        tracing::trace!("\t\tGrid");

        grid_widget.render(grid_area, buf, ctx, &mut state.grid_state());

        // Render all defined sides
        if let Some(mut widget) = top_widget {
            tracing::trace!("\t\tTop");
            widget.render(top_area, buf, ctx, state);
        }
        if let Some(mut widget) = right_widget {
            tracing::trace!("\t\tRight");
            widget.render(right_area, buf, ctx, state);
        }
        if let Some(mut widget) = bottom_widget {
            tracing::trace!("\t\tBottom");
            widget.render(bottom_area, buf, ctx, state);
        }
        if let Some(mut widget) = left_widget {
            tracing::trace!("\t\tLeft");
            widget.render(left_area, buf, ctx, state);
        }
    }

    fn render_size(&self, area: Rect, ctx: &AppContext<A>, state: &mut Self::State) -> Size {
        let mut size = Size::ZERO;

        let grid_widget = GridWidget::new(self.grid);
        let grid_size = grid_widget.render_size(area, ctx, &mut state.grid_state());

        let top_size = self.side_size(Side::Top, self.top, area, ctx, state);
        let right_size = self.side_size(Side::Right, self.right, area, ctx, state);
        let bottom_size = self.side_size(Side::Bottom, self.bottom, area, ctx, state);
        let left_size = self.side_size(Side::Left, self.left, area, ctx, state);

        size.width += grid_size.width;
        size.width += left_size.width;
        size.width += right_size.width;

        size.height += grid_size.height;
        size.height += top_size.height;
        size.height += bottom_size.height;

        size
    }

    fn on_command(
        &mut self,
        command: crate::AppCommand<A>,
        resolver: crate::AppResolver<A>,
        ctx: &mut AppContext<A>,
        state: &mut Self::State,
    ) -> bool {
        let mut result = false;

        match state.focus {
            Some(side @ Side::Top) => {
                if let Some(top) = self.top {
                    let mut widget = SideWidget::new(side, top);
                    result = widget.on_command(command, resolver, ctx, state);
                }
            }
            Some(side @ Side::Right) => {
                if let Some(right) = self.right {
                    let mut widget = SideWidget::new(side, right);
                    result = widget.on_command(command, resolver, ctx, state);
                }
            }
            Some(side @ Side::Bottom) => {
                if let Some(bottom) = self.bottom {
                    let mut widget = SideWidget::new(side, bottom);
                    result = widget.on_command(command, resolver, ctx, state);
                }
            }
            Some(side @ Side::Left) => {
                if let Some(left) = self.left {
                    let mut widget = SideWidget::new(side, left);
                    result = widget.on_command(command, resolver, ctx, state);
                }
            }
            None => {
                let mut widget = GridWidget::new(self.grid);
                result = widget.on_command(command, resolver, ctx, &mut state.grid_state());
            }
        }

        result
    }
}

impl<'a, A, CT, TT, RT, BT, LT, C, E> SidedGridWidget<'a, A, CT, TT, RT, BT, LT, C, E>
where
    A: AppTypes,
    CT: CellRender<A, C>,
    TT: EdgeRender<A, E>,
    RT: EdgeRender<A, E>,
    BT: EdgeRender<A, E>,
    LT: EdgeRender<A, E>,
    C: 'a,
    E: 'a,
{
    fn side_size<U>(
        &self,
        side: Side,
        edges: Option<&'a Vec<U>>,
        area: Rect,
        ctx: &AppContext<A>,
        state: &mut SidedGridWidgetState<'a, C, E>,
    ) -> Size
    where
        U: EdgeRender<A, E>,
    {
        let Some(edges) = edges else {
            return Size::ZERO;
        };

        let widget = SideWidget::<A, U, C, E>::new(side, edges);
        widget.render_size(area, ctx, state)
    }

    fn side_widget<U>(
        &self,
        side: Side,
        edges: Option<&'a Vec<U>>,
    ) -> Option<SideWidget<'a, A, U, C, E>> {
        let edges = edges?;
        let widget = SideWidget::<'a, A, U, C, E>::new(side, edges);

        Some(widget)
    }
}
