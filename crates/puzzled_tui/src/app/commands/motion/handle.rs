use std::fmt::Debug;

use puzzled_core::{
    Direction, Grid, GridIndexedIter, GridLinearIter, GridPositionsIter, Position, Square,
    SquareGridRef,
};

use crate::{GridRenderState, Motion};

pub trait HandleMotion<M, S, S2, P> {
    fn handle_motion(
        &self,
        count: usize,
        motion: Motion<M>,
        state: &mut S,
        custom_state: &mut S2,
    ) -> impl IntoIterator<Item = P>;
}

pub trait HandleCustomMotion<M, S, S2, P> {
    fn handle_custom_motion(
        &self,
        count: usize,
        motion: M,
        state: &mut S,
        custom_state: &mut S2,
    ) -> impl IntoIterator<Item = P>;
}

impl<T, S, S2, P> HandleCustomMotion<(), S, S2, P> for T {
    fn handle_custom_motion(
        &self,
        _count: usize,
        _motion: (),
        _state: &mut S,
        _custom_state: &mut S2,
    ) -> impl IntoIterator<Item = P> {
        std::iter::empty()
    }
}

impl<M, T, S> HandleMotion<M, GridRenderState, S, Position> for Grid<T>
where
    T: Clone + Debug,
    Grid<T>: HandleCustomMotion<M, GridRenderState, S, Position>,
{
    fn handle_motion(
        &self,
        count: usize,
        motion: Motion<M>,
        render: &mut GridRenderState,
        custom_state: &mut S,
    ) -> impl IntoIterator<Item = Position> {
        // Determine where to start the motion from and in which direction
        let start = render.cursor;
        let next_dir = Direction::try_from(&motion).unwrap_or(render.direction);

        // Perform the motion by collecting all covered positions in the grid
        let iter = grid_motion(self, count, motion, start, next_dir, render, custom_state);

        // If currently going in another direction, change the direction
        if render.use_direction
            && !render.mode.is_visual()
            && iter.clone().count() <= 2
            && ![next_dir, !next_dir].contains(&render.direction)
        {
            render.direction = next_dir;
        }
        // Otherwise, move to the end and make sure it stays visible
        else if let Some(end) = iter.clone().next_back().map(|(pos, _)| pos) {
            render.cursor = end;
            render.ensure_cursor_visible(end);

            if let Some(app_end) = render.to_app(end) {
                render.selection.update(app_end);
            }
        }

        iter.map(|(pos, _)| pos)
    }
}

impl<M, T, S> HandleMotion<M, GridRenderState, S, Position> for SquareGridRef<'_, T>
where
    T: Clone + Debug,
    Grid<Square<T>>: HandleCustomMotion<M, GridRenderState, S, Position>,
{
    fn handle_motion(
        &self,
        count: usize,
        motion: Motion<M>,
        render: &mut GridRenderState,
        custom_state: &mut S,
    ) -> impl IntoIterator<Item = Position> {
        // Go to the first filled square in the given direction to perform the motion
        let mut pos = render.cursor;
        let next_dir = Direction::try_from(&motion).unwrap_or(render.direction);

        if count > 0 {
            while let Some(next) = pos + next_dir
                && self.0.is_in_bounds(next)
            {
                // NOTE: since count > 0, some motion always moves the position past the currently non-fill pos
                if self.0.is_fill(next) {
                    break;
                }

                pos = next;
            }
        }

        // Perform the motion by collecting all covered positions in the grid
        let iter = grid_motion(self.0, count, motion, pos, next_dir, render, custom_state);

        // If currently going in another direction, change the direction
        if render.use_direction
            && !render.mode.is_visual()
            && iter.clone().count() <= 2
            && ![next_dir, !next_dir].contains(&render.direction)
        {
            render.direction = next_dir;
        }
        // Otherwise, move to the end and make sure it stays visible
        else if let Some(end) = iter
            .clone()
            .map(|(pos, _)| pos)
            .rfind(|&pos| self.0.is_fill(pos))
        {
            render.cursor = end;
            render.ensure_cursor_visible(end);

            if let Some(app_end) = render.to_app(end) {
                render.selection.update(app_end);
            }
        }

        iter.map(|(pos, _)| pos)
    }
}

fn grid_motion<'a, T, M, S>(
    grid: &'a Grid<T>,
    count: usize,
    motion: Motion<M>,
    start: Position,
    dir: Direction,
    state: &mut GridRenderState,
    custom_state: &mut S,
) -> GridIndexedIter<'a, T>
where
    Grid<T>: HandleCustomMotion<M, GridRenderState, S, Position>,
    T: Debug,
{
    let iter_remaining =
        |remaining: usize| GridLinearIter::new_with_remaining(grid, start, dir.into(), remaining);
    let iter_direction = |dir: Direction| grid.iter_segment(start, dir);

    let iter = match motion {
        Motion::Mouse(mouse) => {
            let app_pos = ratatui::layout::Position {
                x: mouse.column,
                y: mouse.row,
            };

            match state.to_grid(app_pos) {
                Some(pos) => GridLinearIter::new_single(grid, pos),
                None => GridLinearIter::new_empty(grid),
            }
        }
        Motion::Down | Motion::Left | Motion::Right | Motion::Up => iter_remaining(count + 1),
        Motion::RowStart | Motion::RowEnd | Motion::ColStart | Motion::ColEnd => {
            iter_direction(dir)
        }
        Motion::Custom(custom) => {
            let positions: Vec<_> = grid
                .handle_custom_motion(count, custom, state, custom_state)
                .into_iter()
                .collect();
            let iter = GridPositionsIter::new(grid, positions);

            return GridIndexedIter::new_positions(iter);
        }
        _ => GridLinearIter::new_empty(grid),
    };

    GridIndexedIter::new_linear(iter)
}
