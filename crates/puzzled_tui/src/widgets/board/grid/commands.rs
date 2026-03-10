use std::fmt::Debug;

use puzzled_core::{Direction, Grid, Position, Solve, SquareGridRef};

use crate::{
    ActionResolver, Command, GridRenderState, HandleCommand, Motion, MotionRange, Operator,
};

pub struct SolveBoard<'a, B, S> {
    board: &'a mut B,
    state: &'a mut S,
}

impl<'a, M, A, S, B, Z> HandleCommand<M, A, S> for SolveBoard<'a, B, Z>
where
    B: HandleCommand<M, A, S, State = GridRenderState>,
    Z: Solve<Position = Position>,
{
    type State = GridRenderState;

    fn on_command(
        &mut self,
        command: Command<M, A>,
        resolver: ActionResolver<M, A, S>,
        state: &mut Self::State,
    ) -> bool {
        let pos = state.cursor;

        let Some(motion) = command.motion() else {
            return false;
        };

        let Some(op) = command.operator() else {
            return self.board.on_command(command, resolver, state);
        };

        match op {
            Operator::Reveal => {
                self.state.reveal(&pos);
            }
            Operator::Check => {
                self.state.check(&pos);
            }
            _ => {}
        }

        true
    }
}

impl<M, A, S, T> HandleCommand<M, A, S> for Grid<T> {
    type State = GridRenderState;

    fn on_command(
        &mut self,
        command: Command<M, A>,
        _resolver: ActionResolver<M, A, S>,
        state: &mut Self::State,
    ) -> bool {
        let count = command.count();
        let Some(motion) = command.motion() else {
            return false;
        };

        // Positions
        let start = state.cursor;

        let positions: Vec<_> = self
            .motion_range(start, count, motion)
            .into_iter()
            .collect();

        let end = positions.last().cloned().unwrap_or(start);
        let direction = Direction::try_from(motion).unwrap_or(state.direction);

        if start != end {
            state.cursor = end;
            state.direction += direction;
            state.ensure_cursor_visible();
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
        let Some(motion) = command.motion() else {
            return false;
        };

        let start = state.cursor;
        let dir = state.direction;

        let positions: Vec<_> = self
            .motion_range(start, count, motion)
            .into_iter()
            .collect(); // Pass count to motion_range

        tracing::info!("Motion range positions: {positions:?}");

        // Take the last position in the range (after all the steps)
        let next_dir = Direction::try_from(motion).unwrap_or(state.direction);
        let (end, next_dir) = move_in_dir(self, start, dir, next_dir, &positions);

        tracing::info!("Dir 1");

        if (start, dir) != (end, next_dir) {
            state.cursor = end;
            state.direction = next_dir;
            state.ensure_cursor_visible();
        }

        true
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
