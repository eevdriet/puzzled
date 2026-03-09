use std::fmt::Debug;

use puzzled_core::{Direction, Grid, Position, SquareGridRef};

use crate::{ActionResolver, Command, GridRenderState, HandleCommand, Motion};

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

        // Bounds
        let max_row = self.rows() - 1;
        let max_col = self.cols() - 1;

        // Positions
        let start = state.cursor;
        let Position { col, row } = start;

        // Determine the end position of cursor movements

        let end: Position = match motion {
            // -- Movements --
            // Left
            Motion::Left => Position {
                col: col.saturating_sub(count),
                ..start
            },
            // Right
            Motion::Right => Position {
                col: (col + count).min(max_col),
                ..start
            },
            // Up
            Motion::Up => Position {
                row: row.saturating_sub(count),
                ..start
            },
            // Down
            Motion::Down => Position {
                row: (row + count).min(max_row),
                ..start
            },

            // Column
            Motion::Col(col) => Position { col: *col, ..start },
            Motion::ColEnd => Position {
                row: max_row,
                ..start
            },
            Motion::ColStart => Position { row: 0, ..start },

            // Row
            Motion::Row(row) => Position { row: *row, ..start },
            Motion::RowEnd => Position {
                col: max_col,
                ..start
            },
            Motion::RowStart => Position { col: 0, ..start },

            // Mouse
            // Action::Click(mouse) | Action::Drag(mouse) => match state.to_grid(mouse) {
            //     None => start,
            //     Some(pos) => pos,
            // },
            _ => return false,
        };

        let direction = match motion {
            Motion::Up => Direction::Up,
            Motion::Down => Direction::Down,
            Motion::Left => Direction::Left,
            Motion::Right => Direction::Right,
            _ => state.direction,
        };

        if start != end {
            state.cursor = end;
            state.direction += direction;
        }

        true
    }
}

impl<M, A, S, T> HandleCommand<M, A, S> for SquareGridRef<'_, T> {
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
        let Position { col, row } = start;
        let dir = state.direction;

        let mut move_dir = |dir: Direction, count: usize| {
            move_in_dir(self, state.cursor, state.direction, dir, count)
        };

        let (next, next_dir) = match motion {
            // Direct moves
            Motion::Down => move_dir(Direction::Down, count),
            Motion::Left => move_dir(Direction::Left, count),
            Motion::Right => move_dir(Direction::Right, count),
            Motion::Up => move_dir(Direction::Up, count),

            // Column
            Motion::Col(col) => {
                let next = self
                    .0
                    .iter_indexed_col(*col)
                    .find(|&(_, square)| square.is_some())
                    .map(|(pos, _)| pos)
                    .unwrap_or(start);

                (next, dir)
            }
            Motion::ColEnd => {
                let next = self
                    .0
                    .iter_indexed_col(col)
                    .rev()
                    .find(|&(_, square)| square.is_some())
                    .map(|(pos, _)| pos)
                    .unwrap_or(start);
                (next, dir)
            }
            Motion::ColStart => {
                let next = self
                    .0
                    .iter_indexed_col(col)
                    .find(|&(_, square)| square.is_some())
                    .map(|(pos, _)| pos)
                    .unwrap_or(start);

                (next, dir)
            }

            // Row
            Motion::Row(row) => {
                let next = self
                    .0
                    .iter_indexed_row(*row)
                    .find(|&(_, square)| square.is_some())
                    .map(|(pos, _)| pos)
                    .unwrap_or(start);

                (next, dir)
            }
            Motion::RowEnd => {
                let next = self
                    .0
                    .iter_indexed_row(row)
                    .rev()
                    .find(|&(_, square)| square.is_some())
                    .map(|(pos, _)| pos)
                    .unwrap_or(start);
                (next, dir)
            }
            Motion::RowStart => {
                let next = self
                    .0
                    .iter_indexed_row(row)
                    .find(|&(_, square)| square.is_some())
                    .map(|(pos, _)| pos)
                    .unwrap_or(start);

                (next, dir)
            }

            _ => return false,
        };

        state.cursor = next;
        state.direction = next_dir;

        true
    }
}

fn move_in_dir<'a, T>(
    grid: &mut SquareGridRef<'a, T>,
    curr_pos: Position,
    curr_dir: Direction,
    dir: Direction,
    count: usize,
) -> (Position, Direction) {
    let mut pos = curr_pos;

    // If currently going in another direction, change the direction
    if ![dir, !dir].contains(&curr_dir) {
        return (pos, dir);
    }

    // Continue moving in the direction until out of the grid or the correct number of moves
    let mut move_count = 0;

    while let Some(next) = pos + dir
        && grid.0.is_in_bounds(next)
        && move_count < count
    {
        match grid.0.get_fill(next) {
            // Move to next playable square
            Some(_) => {
                pos = next;
                move_count += 1;
            }
            // Move over directly adjacent non-playable squares
            None if move_count == 0 => {
                pos = next;
            }
            // Stop if a non-adjacent non-playable square is found
            None => break,
        }
    }

    let end = if move_count == 0 { curr_pos } else { pos };
    (end, dir)
}
