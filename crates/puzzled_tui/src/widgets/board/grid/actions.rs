use puzzled_core::Position;

use crate::{Action, ActionResolver, GridRenderState, GridWidget, HandleAction};

impl<'a, A, S, T> HandleAction<A, S> for GridWidget<'a, T> {
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

        if start != end {
            state.cursor = end;
        }
    }
}
