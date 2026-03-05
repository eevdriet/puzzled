use std::fmt::Debug;

use puzzled_core::Position;
use ratatui::layout::Position as AppPosition;

use crate::{Action, ActionOutcome, GridState, GridWidget, HandleAction};

impl<'a, A, T> HandleAction<A> for GridWidget<'a, T>
where
    A: Debug,
{
    type State = GridState;
    type Error = ();

    fn handle_action(
        &self,
        action: Action<A>,
        repeat: usize,
        state: &mut Self::State,
    ) -> Result<ActionOutcome<Self::State>, Self::Error> {
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
            Action::MoveLeft | Action::ScrollLeft(_) => Position {
                col: col.saturating_sub(repeat),
                ..start
            },
            // Right
            Action::MoveRight | Action::ScrollRight(_) => Position {
                col: (col + repeat).min(max_col),
                ..start
            },
            // Up
            Action::MoveUp | Action::ScrollUp(_) => Position {
                row: row.saturating_sub(repeat),
                ..start
            },
            // Down
            Action::MoveDown | Action::ScrollDown(_) => Position {
                row: (row + repeat).min(max_row),
                ..start
            },

            // Column
            Action::MoveCol(col) => Position { col, ..start },
            Action::MoveColEnd => Position {
                col: max_col,
                ..start
            },
            Action::MoveColStart => Position { col: 0, ..start },

            // Row
            Action::MoveRow(row) => Position { row, ..start },
            Action::MoveRowEnd => Position {
                row: max_row,
                ..start
            },
            Action::MoveRowStart => Position { row: 0, ..start },

            // Mouse
            Action::Click(mouse) | Action::Drag(mouse) => {
                let end = AppPosition::new(mouse.column, mouse.row);

                match state.to_grid(end) {
                    None => return Ok(ActionOutcome::Ignored),
                    Some(pos) => pos,
                }
            }

            _ => start,
        };

        if start != end {
            tracing::info!("{action:?}: {start} -> {end}");

            state.cursor = end;
        }
        Ok(ActionOutcome::Consumed)
    }
}
