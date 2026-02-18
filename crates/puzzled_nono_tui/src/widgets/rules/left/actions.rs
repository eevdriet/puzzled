use crossterm::event::Event;
use nono::Position;

use crate::{
    Action, ActionInput, ActionOutcome, AppState, Error, HandleAction, MotionRange, Result,
    RowRulesWidget, app_to_puzzle, puzzle_to_app,
    widgets::rules::actions::{handle_command, handle_mouse},
};

impl HandleAction for &RowRulesWidget {
    fn handle_command(&self, input: ActionInput, state: &mut AppState) -> crate::ActionResult {
        handle_command(input, state)
    }

    fn handle_motion(
        &self,
        input: ActionInput,
        state: &mut AppState,
    ) -> Result<(ActionOutcome, Option<MotionRange>)> {
        let fill_regions = &state.rules_left.fill_regions;
        let rule_state = &state.rules_left;

        let event = input.event;
        let action = input.action;
        let count = input.repeat.unwrap_or(1);

        // Lose focus commands
        let get_max_col = |row: u16| -> u16 {
            let rule = &rule_state.rules[row as usize];
            rule.runs().len() as u16 - 1
        };

        let next_back_idx = |col: u16, row: u16, next_row: u16| -> u16 {
            let back = get_max_col(row).saturating_sub(col);

            get_max_col(next_row).saturating_sub(back)
        };

        let pos: Position = app_to_puzzle(rule_state.cursor);
        let col = pos.col;
        let row = pos.row;

        let max_row = state.puzzle.puzzle.rows() - 1;
        let max_col = get_max_col(row);

        let (end, produce_motion) = match action {
            Action::MoveLeft
            | Action::ScrollLeft
            | Action::JumpStartBackwards
            | Action::JumpEndBackwards => (
                Position {
                    col: col.saturating_sub(count),
                    ..pos
                },
                true,
            ),
            Action::MoveRight
            | Action::ScrollRight
            | Action::JumpStartForwards
            | Action::JumpEndForwards => (
                Position {
                    col: (col + count).min(max_col),
                    ..pos
                },
                true,
            ),
            Action::MoveUp | Action::ScrollUp => {
                let next_row = row.saturating_sub(count);
                let next_col = next_back_idx(col, row, next_row);

                (
                    Position {
                        row: next_row,
                        col: next_col,
                    },
                    false,
                )
            }
            Action::MoveDown | Action::ScrollDown => {
                let next_row = (row + count).min(max_row);
                let next_col = next_back_idx(col, row, next_row);

                (
                    Position {
                        row: next_row,
                        col: next_col,
                    },
                    false,
                )
            }

            // Line jumps
            Action::JumpRowStart => (Position { col: 0, ..pos }, true),
            Action::JumpRowEnd => (
                Position {
                    col: max_col,
                    ..pos
                },
                true,
            ),
            Action::JumpColStart => (
                Position {
                    row: 0,
                    col: next_back_idx(col, row, 0),
                },
                false,
            ),
            Action::JumpColEnd => (
                Position {
                    row: max_row,
                    col: next_back_idx(col, row, max_row),
                },
                false,
            ),

            Action::Click => {
                let Event::Mouse(mouse) = *event else {
                    return Err(Error::Custom(format!(
                        "Found invalid event {event:?} for {action:?}"
                    )));
                };

                handle_mouse(fill_regions, mouse, &mut state.puzzle.fill);
                (pos, false)
            }

            _ => (pos, false),
        };

        tracing::info!("{pos:?} -> {end:?}");

        let cursor = puzzle_to_app(end);
        state.rules_left.cursor = cursor;

        state.puzzle.cursor.y = cursor.y;
        state.puzzle.keep_cursor_visible(state.puzzle.cursor);

        Ok((
            ActionOutcome::Consumed,
            produce_motion.then_some(MotionRange::Single(cursor)),
        ))
    }
}
