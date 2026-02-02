use nono::{Position, Result};

use crate::{
    Action, ActionInput, ActionOutcome, AppState, ColRulesWidget, HandleAction, MotionRange,
    app_to_puzzle, puzzle_to_app, widgets::rules::actions::handle_command,
};

impl HandleAction for &ColRulesWidget {
    fn handle_command(&self, input: ActionInput, state: &mut AppState) -> crate::ActionResult {
        handle_command(input, state)
    }

    fn handle_motion(
        &self,
        input: ActionInput,
        state: &mut AppState,
    ) -> Result<(ActionOutcome, Option<MotionRange>)> {
        let rule_state = &mut state.rules_top;

        let action = input.action;
        let count = input.repeat.unwrap_or(1);

        // Lose focus commands
        let get_max_row = |col: u16| -> u16 {
            let rule = &rule_state.rules[col as usize];
            rule.runs().len() as u16 - 1
        };

        let pos: Position = app_to_puzzle(rule_state.cursor);
        let col = pos.col;
        let row = pos.row;

        let max_row = get_max_row(col);
        let max_col = state.puzzle.puzzle.cols() - 1;

        // Lose focus commands
        let (end, produce_motion) = match action {
            Action::MoveLeft => {
                let col = col.saturating_sub(count);
                let row = row.min(get_max_row(col));

                (Position { row, col }, false)
            }
            Action::MoveRight => {
                let col = (col + count).min(max_col);
                let row = row.min(get_max_row(col));

                (Position { row, col }, false)
            }
            Action::MoveUp => (
                Position {
                    row: row.saturating_sub(count),
                    ..pos
                },
                true,
            ),
            Action::MoveDown => (
                Position {
                    row: (row + count).min(max_row),
                    ..pos
                },
                true,
            ),
            _ => (pos, false),
        };

        let cursor = puzzle_to_app(end);
        rule_state.cursor = cursor;

        state.puzzle.cursor.x = cursor.x;
        state.puzzle.keep_cursor_visible(state.puzzle.cursor);

        Ok((
            ActionOutcome::Consumed,
            produce_motion.then_some(MotionRange::Single(cursor)),
        ))
    }
}
