use puzzled_core::{Direction, Grid, GridIndexedIter, GridIter, Position, SquareGridRef};

use crate::{GridRenderState, Motion};

pub trait HandleBaseMotion<M, S> {
    type Position;

    fn handle_base_motion(
        &self,
        count: usize,
        motion: Motion<M>,
        state: &mut S,
    ) -> impl IntoIterator<Item = Self::Position>;
}

pub trait HandleCustomMotion<M, S> {
    type Position;

    fn handle_base_motion(
        &self,
        count: usize,
        motion: M,
        state: &mut S,
    ) -> impl IntoIterator<Item = Self::Position>;
}

impl<M, T> HandleBaseMotion<M, GridRenderState> for Grid<T>
where
    T: Clone,
{
    type Position = Position;

    fn handle_base_motion(
        &self,
        count: usize,
        motion: Motion<M>,
        state: &mut GridRenderState,
    ) -> impl IntoIterator<Item = Self::Position> {
        // Determine where to start the motion from and in which direction
        let start = state.cursor;
        let next_dir = Direction::try_from(&motion).unwrap_or(state.direction);

        // Perform the motion by collecting all covered positions in the grid
        let iter = grid_motion(self, count, motion, start, next_dir, state);

        // If currently going in another direction, change the direction
        if iter.clone().count() <= 2 && ![next_dir, !next_dir].contains(&state.direction) {
            state.direction = next_dir;
        }
        // Otherwise, move to the end and make sure it stays visible
        else if let Some(end) = iter.clone().next_back().map(|(pos, _)| pos) {
            state.cursor = end;
            state.ensure_cursor_visible(end);
        }

        iter.map(|(pos, _)| pos)
    }
}

impl<M, T> HandleBaseMotion<M, GridRenderState> for SquareGridRef<'_, T>
where
    T: Clone,
{
    type Position = Position;

    fn handle_base_motion(
        &self,
        count: usize,
        motion: Motion<M>,
        state: &mut GridRenderState,
    ) -> impl IntoIterator<Item = Self::Position> {
        // Go to the first filled square in the given direction to perform the motion
        let mut pos = state.cursor;
        let next_dir = Direction::try_from(&motion).unwrap_or(state.direction);

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
        let iter = grid_motion(self.0, count, motion, pos, next_dir, state);

        // If currently going in another direction, change the direction
        if iter.clone().count() <= 2 && ![next_dir, !next_dir].contains(&state.direction) {
            state.direction = next_dir;
        }
        // Otherwise, move to the end and make sure it stays visible
        else if let Some(end) = iter
            .clone()
            .map(|(pos, _)| pos)
            .rfind(|&pos| self.0.is_fill(pos))
        {
            state.cursor = end;
            state.ensure_cursor_visible(end);
        }

        iter.map(|(pos, _)| pos)
    }
}

fn grid_motion<'a, T, M>(
    grid: &'a Grid<T>,
    count: usize,
    motion: Motion<M>,
    start: Position,
    dir: Direction,
    state: &GridRenderState,
) -> GridIndexedIter<'a, T> {
    let iter_remaining =
        |remaining: usize| GridIter::new_with_remaining(grid, start, dir.into(), remaining);
    let iter_direction = |dir: Direction| grid.iter_segment(start, dir);

    let iter = match motion {
        Motion::Mouse(mouse) => {
            let app_pos = ratatui::layout::Position {
                x: mouse.column,
                y: mouse.row,
            };

            match state.to_grid(app_pos) {
                Some(pos) => GridIter::new_single(grid, pos),
                None => GridIter::new_empty(grid),
            }
        }
        Motion::None => iter_remaining(1),
        Motion::Down | Motion::Left | Motion::Right | Motion::Up => iter_remaining(count + 1),
        Motion::RowStart | Motion::RowEnd | Motion::ColStart | Motion::ColEnd => {
            iter_direction(dir)
        }
        _ => GridIter::new_empty(grid),
    };

    GridIndexedIter::new(iter)
}
