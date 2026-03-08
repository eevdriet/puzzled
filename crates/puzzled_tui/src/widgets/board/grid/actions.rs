use puzzled_core::{Direction, Grid, Position, SquareGridRef, Value};

use crate::{Action, ActionResolver, GridRenderState, HandleAction};

impl<A, S, T> HandleAction<A, S> for Grid<T> {
    type State = GridRenderState;

    fn on_action(
        &mut self,
        action: Action<A>,
        _resolver: ActionResolver<A, S>,
        state: &mut Self::State,
    ) {
        // Bounds
        let max_row = self.rows() - 1;
        let max_col = self.cols() - 1;

        // Positions
        let start = state.cursor;
        let Position { col, row } = start;

        // Determine the end position of cursor movements
        let end: Position = match action {
            // -- Movements --
            // Left
            Action::MoveLeft(count) => Position {
                col: col.saturating_sub(count),
                ..start
            },
            // Right
            Action::MoveRight(count) => Position {
                col: (col + count).min(max_col),
                ..start
            },
            // Up
            Action::MoveUp(count) => Position {
                row: row.saturating_sub(count),
                ..start
            },
            // Down
            Action::MoveDown(count) => Position {
                row: (row + count).min(max_row),
                ..start
            },

            // Column
            Action::MoveCol(col) => Position { col, ..start },
            Action::MoveColEnd => Position {
                row: max_row,
                ..start
            },
            Action::MoveColStart => Position { row: 0, ..start },

            // Row
            Action::MoveRow(row) => Position { row, ..start },
            Action::MoveRowEnd => Position {
                col: max_col,
                ..start
            },
            Action::MoveRowStart => Position { col: 0, ..start },

            // Mouse
            Action::Click(mouse) | Action::Drag(mouse) => match state.to_grid(mouse) {
                None => start,
                Some(pos) => pos,
            },

            _ => start,
        };

        let direction = match action {
            Action::MoveUp(_) => Direction::Up,
            Action::MoveDown(_) => Direction::Down,
            Action::MoveLeft(_) => Direction::Left,
            Action::MoveRight(_) => Direction::Right,
            _ => state.direction,
        };

        if start != end {
            state.cursor = end;
            state.direction += direction;
        }
    }
}

impl<A, S, T> HandleAction<A, S> for SquareGridRef<'_, T> {
    type State = GridRenderState;

    fn on_action(
        &mut self,
        action: Action<A>,
        _resolver: ActionResolver<A, S>,
        state: &mut Self::State,
    ) {
        let move_in_dir = |dir: Direction, count: usize| {
            let mut pos = state.cursor;
            let curr_dir = state.direction;

            // If currently going in another direction, change the direction
            if dir != curr_dir {
                return (pos, dir);
            }

            for _step_count in 1..=count {
                // Verify the next square can be reached and it is playable
                let Some(next) = pos + dir else {
                    break;
                };

                if self.0.get_fill(next).is_none() {
                    break;
                }

                // If so, go there, otherwise stop at the last playable square
                pos = next;
            }

            (pos, dir)
        };

        let (next, next_dir) = match action {
            Action::MoveDown(count) => move_in_dir(Direction::Down, count),
            Action::MoveLeft(count) => move_in_dir(Direction::Left, count),
            Action::MoveRight(count) => move_in_dir(Direction::Right, count),
            Action::MoveUp(count) => move_in_dir(Direction::Up, count),
            _ => (state.cursor, state.direction),
        };
        state.cursor = next;
        state.direction = next_dir;
    }
}
