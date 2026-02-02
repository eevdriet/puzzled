mod fill;

pub use fill::*;

use crossterm::event::Event;
use nono::{Error, Fill, FindDirection, LinePosition, Position, Puzzle, Result};
use ratatui::layout::Position as AppPosition;

use crate::{
    Action, ActionInput, ActionOutcome, ActionResult, AppState, HandleAction, MotionRange,
    PuzzleWidget, app_to_puzzle, puzzle_to_app,
};

// H
// M
// L
// <
// >

impl HandleAction for &PuzzleWidget {
    fn handle_operator(
        &self,
        input: ActionInput,
        range: Option<MotionRange>,
        state: &mut AppState,
    ) -> ActionResult {
        let action = input.action;

        let fill = match action {
            Action::Delete | Action::DeleteSingle => Fill::Blank,
            Action::Cross => Fill::Cross,
            Action::Fill => state.puzzle.fill,
            _ => {
                return Err(Error::Custom(format!(
                    "Found invalid action {:?}",
                    input.action
                )));
            }
        };

        let range = match range {
            Some(range) => range,
            None => MotionRange::Single(state.puzzle.cursor),
        };

        // Track which fills should be changed
        let bounds = state.puzzle.bounds();
        let mut changes = Vec::new();

        for pos in range.positions(&bounds) {
            let pos = app_to_puzzle(pos);
            let before = state.puzzle.puzzle[pos];

            // Only record actual changes
            if before != fill {
                let change = CellChange::new(pos, before, fill);
                changes.push(change);
            }
        }

        if changes.is_empty() {
            return Ok(ActionOutcome::Consumed);
        }

        let cmd = FillAction::new(changes);
        Ok(ActionOutcome::Command(Box::new(cmd)))
    }

    fn handle_motion(
        &self,
        input: ActionInput,
        state: &mut AppState,
    ) -> Result<(ActionOutcome, Option<MotionRange>)> {
        let puzzle = &state.puzzle.puzzle;

        // Input
        let event = input.event;
        let action = input.action;
        let count = input.repeat.unwrap_or(1);

        // Bounds
        let max_row = puzzle.rows() - 1;
        let max_col = puzzle.cols() - 1;
        let vp = &state.puzzle.viewport;

        // Positions
        let pos: Position = app_to_puzzle(state.puzzle.cursor);
        let col = pos.col;
        let row = pos.row;
        let axis_pos = pos.along_axis(state.puzzle.motion_axis);

        let end: Position = match action {
            // Moves
            Action::MoveLeft | Action::ScrollLeft => Position {
                col: col.saturating_sub(count),
                ..pos
            },
            Action::MoveRight | Action::ScrollRight => Position {
                col: (col + count).min(max_col),
                ..pos
            },
            Action::MoveUp | Action::ScrollUp => Position {
                row: row.saturating_sub(count),
                ..pos
            },
            Action::MoveDown | Action::ScrollDown => Position {
                row: (row + count).min(max_row),
                ..pos
            },

            // Fill finds
            Action::FindFillForwards => puzzle
                .find_fill(axis_pos, state.puzzle.fill, FindDirection::Forwards)
                .map(|pos| pos.into())
                .unwrap_or(pos),

            Action::FindFillBackwards => puzzle
                .find_fill(axis_pos, state.puzzle.fill, FindDirection::Backwards)
                .map(|pos| pos.into())
                .unwrap_or(pos),

            Action::FindTilFillForwards => puzzle
                .find_fill(axis_pos, state.puzzle.fill, FindDirection::Forwards)
                .map(|pos| (pos - 1).into())
                .unwrap_or(pos),

            Action::FindTilFillBackwards => puzzle
                .find_fill(axis_pos, state.puzzle.fill, FindDirection::Backwards)
                .map(|pos| (pos + 1).into())
                .unwrap_or(pos),

            // Line jumps
            Action::JumpRowStart => Position { col: 0, ..pos },
            Action::JumpRowEnd => Position {
                col: max_col,
                ..pos
            },

            // Jump to the start/end row without repeat (e.g. G)
            Action::JumpColStart if input.repeat.is_none() => Position { row: 0, ..pos },

            Action::JumpColEnd if input.repeat.is_none() => Position {
                row: max_row,
                ..pos
            },

            // Jump to specific line with repeat (e.g. 5G)
            Action::JumpColStart | Action::JumpColEnd if input.repeat.is_some() => Position {
                row: count.saturating_sub(1),
                ..pos
            },

            Action::JumpCol if input.repeat.is_some() => Position {
                col: count.saturating_sub(1),
                ..pos
            },

            // Jump to non-blank runs
            Action::JumpFirstNonBlank => puzzle
                .find_first_non_blank_fill(axis_pos.line, FindDirection::Forwards)
                .map(|pos| pos.into())
                .unwrap_or(pos),

            Action::JumpLastNonBlank => puzzle
                .find_first_non_blank_fill(axis_pos.line, FindDirection::Backwards)
                .map(|pos| pos.into())
                .unwrap_or(pos),

            // Run jumps
            Action::JumpStartForwards => {
                handle_jumps(puzzle, axis_pos, true, FindDirection::Forwards, count)
            }
            Action::JumpStartBackwards => {
                handle_jumps(puzzle, axis_pos, true, FindDirection::Backwards, count)
            }
            Action::JumpEndForwards => {
                handle_jumps(puzzle, axis_pos, false, FindDirection::Forwards, count)
            }
            Action::JumpEndBackwards => {
                handle_jumps(puzzle, axis_pos, false, FindDirection::Backwards, count)
            }

            // Cell jumps
            Action::Click | Action::Drag => {
                let Event::Mouse(mouse) = *event else {
                    return Err(Error::Custom(format!(
                        "Found invalid event {event:?} for {action:?}"
                    )));
                };

                let end = AppPosition::new(mouse.column, mouse.row);
                if vp.area.contains(end) {
                    state.puzzle.screen_to_puzzle(vp.area, end).unwrap_or(pos)
                } else {
                    return Ok((ActionOutcome::Ignored, None));
                }
            }

            _ => pos,
        };

        tracing::debug!("Puzzle end: {end:?}");
        let cursor = puzzle_to_app(end);
        tracing::debug!("App end: {cursor:?}");

        state.rules_left.follow_puzzle_cursor(end);
        state.rules_top.follow_puzzle_cursor(end);

        tracing::debug!("End position: {cursor:?}");
        state.puzzle.cursor = cursor;
        state.puzzle.keep_cursor_visible(cursor);

        Ok((ActionOutcome::Consumed, Some(MotionRange::Single(cursor))))
    }

    fn handle_command(&self, input: ActionInput, state: &mut AppState) -> ActionResult {
        let action = input.action;

        let vp = &state.puzzle.viewport;
        let _y_scroll_max = state.puzzle.puzzle.rows() - vp.area.height;
        let _y_half = vp.area.height / 2;

        if matches!(action, Action::SwitchFill)
            && let Event::Key(key) = *input.event
            && let Some(fill) = state.puzzle.style.fill_from_key(key)
        {
            state.puzzle.fill = fill;
            return Ok(ActionOutcome::Consumed);
        }

        match action {
            Action::FocusDown | Action::FocusUp | Action::FocusLeft | Action::FocusRight => {
                return Ok(ActionOutcome::LoseFocus);
            }
            Action::SampleFill => {
                let pos = app_to_puzzle(state.puzzle.cursor);
                let fill = state.puzzle.puzzle[pos];
                state.puzzle.fill = fill;
            }

            Action::SwitchAxis => {
                state.puzzle.motion_axis.switch();
                state.puzzle.selection.axis.switch();
            }

            // TODO: Implement properly by changing scroll too
            Action::TopViewport => {
                // state.puzzle.scroll.row = state.puzzle.cursor.y.min(y_scroll_max);
            }
            Action::BottomViewport => {
                // if state.puzzle.cursor.y < y_scroll_max {
                //     state.puzzle.cursor.y = y_scroll_max;
                // } else {
                //     state.puzzle.scroll.row = state
                //         .puzzle
                //         .cursor
                //         .y
                //         .saturating_sub(visible.height)
                //         .min(y_scroll_max);
                // }
            }
            Action::CenterViewport => {
                // if state.puzzle.cursor.y < y_half {
                //     state.puzzle.cursor.y = y_half;
                // } else {
                //     state.puzzle.scroll.row = state
                //         .puzzle
                //         .cursor
                //         .y
                //         .saturating_sub(y_half)
                //         .min(y_scroll_max);
                // }
            }
            _ => {
                return Ok(ActionOutcome::Ignored);
            }
        }

        Ok(ActionOutcome::Consumed)
    }
}

fn handle_jumps(
    puzzle: &Puzzle,
    pos: LinePosition,
    to_start: bool,
    direction: FindDirection,
    count: u16,
) -> Position {
    let mut pos = pos;

    for _ in 0..count {
        // Try to jump to the next position
        let next_pos = match to_start {
            true => puzzle.find_directed_run_start(pos, direction),
            false => puzzle.find_directed_run_end(pos, direction),
        };

        // If not possible, the start/end is reached: stop
        let Some(next_pos) = next_pos else {
            break;
        };

        pos = next_pos;
    }

    pos.into()
}
