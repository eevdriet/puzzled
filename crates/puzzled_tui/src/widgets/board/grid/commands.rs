use std::fmt::Debug;

use puzzled_core::{Direction, Grid, Position, SquareGridRef};

use crate::{ActionResolver, BaseMotionRange, Command, GridRenderState, HandleCommand};

impl<M, A, S, T> HandleCommand<M, A, S> for Grid<T> {
    type State = GridRenderState;

    fn on_command(
        &mut self,
        command: Command<M, A>,
        _resolver: ActionResolver<M, A, S>,
        state: &mut Self::State,
    ) -> bool {
        let count = command.count();
        let motion = command.motion();

        // Positions
        let start = state.cursor;

        let positions: Vec<_> = self
            .base_motion_range(start, count, motion)
            .into_iter()
            .collect();

        let end = positions.last().cloned().unwrap_or(start);
        let direction = Direction::try_from(motion).unwrap_or(state.direction);

        if start != end {
            state.cursor = end;
            state.direction += direction;
            state.ensure_cursor_visible(end);
        }

        true
    }
}

impl<M, A, S, T> HandleCommand<M, A, S> for SquareGridRef<'_, T>
where
    M: Debug,
{
    type State = GridRenderState;

    fn on_command(
        &mut self,
        command: Command<M, A>,
        _resolver: ActionResolver<M, A, S>,
        state: &mut Self::State,
    ) -> bool {
        let count = command.count();
        let motion = command.motion();

        let start = state.cursor;
        let dir = state.direction;

        let positions: Vec<_> = self
            .base_motion_range(start, count, motion)
            .into_iter()
            .collect(); // Pass count to motion_range

        // Take the last position in the range (after all the steps)
        let next_dir = Direction::try_from(motion).unwrap_or(state.direction);
        let (end, next_dir) = move_in_dir(self, start, dir, next_dir, &positions);

        if (start, dir) != (end, next_dir) {
            state.cursor = end;
            state.direction = next_dir;

            if let Some(last) = positions.last() {
                state.ensure_cursor_visible(*last);
            }

            return true;
        }

        false
    }
}

fn move_in_dir<'a, T>(
    grid: &mut SquareGridRef<'a, T>,
    start: Position,
    dir: Direction,
    next_dir: Direction,
    positions: &[Position],
) -> (Position, Direction) {
    // If currently going in anothr direction, change the direction
    if positions.len() <= 2 && ![next_dir, !next_dir].contains(&dir) {
        return (start, next_dir);
    }

    // Otherwise, find the last valid position to move to
    let end = positions
        .iter()
        .rfind(|&&pos| grid.0.is_fill(pos))
        .copied()
        .unwrap_or(start);
    (end, dir)
}
