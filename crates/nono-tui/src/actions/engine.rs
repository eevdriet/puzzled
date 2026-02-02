use nono::Error;
use ratatui::layout::Position as AppPosition;

use crate::{
    Action, ActionInput, ActionKind, ActionOutcome, AppState, Focus, HandleAction, History, Mode,
    MotionRange, SelectionKind,
};

use super::ActionResult;

#[derive(Debug, Default)]
pub struct ActionEngine {
    pending_operator: Option<Action>,
    pending_motion: Option<Action>,
    history: History,
    mode: Mode,
}

impl ActionEngine {
    pub fn handle_action_with<H: HandleAction>(
        &mut self,
        handler: H,
        input: ActionInput,
        state: &mut AppState,
    ) -> ActionResult {
        let is_visual = matches!(self.mode, Mode::Visual(_));
        let is_normal = matches!(self.mode, Mode::Normal);

        match input.action {
            // Exit application
            Action::Quit => return Ok(ActionOutcome::Exit),

            // Clicks end visual mode
            Action::Click if is_visual => {
                self.exit_visual(state);
                self.mode = Mode::Normal;
            }

            // History management
            Action::Undo if is_normal => return self.history.undo(state),
            Action::Redo if is_normal => return self.history.redo(state),

            // Drag starts visual mode
            Action::Drag if !is_visual => {
                self.enter_visual(SelectionKind::Cells, state.cursor(), state);
            }
            _ => {
                return match self.mode {
                    Mode::Normal => self.handle_normal(handler, input, state),
                    Mode::Visual(_) => self.handle_visual(handler, input, state),
                    Mode::Insert => self.handle_insert(handler, input, state),
                };
            }
        }

        Ok(ActionOutcome::Consumed)
    }

    fn handle_operator<H: HandleAction>(
        &mut self,
        handler: H,
        input: ActionInput,
        range: Option<MotionRange>,
        state: &mut AppState,
    ) -> ActionResult {
        if !input.action.is_motionless_op() {
            self.pending_operator = Some(input.action);
            return Ok(ActionOutcome::Consumed);
        }

        match handler.handle_operator(input, range, state) {
            Ok(ActionOutcome::Command(cmd)) => self.history.execute(cmd, state),
            result => result,
        }
    }

    fn handle_normal<H: HandleAction>(
        &mut self,
        handler: H,
        input: ActionInput,
        state: &mut AppState,
    ) -> ActionResult {
        let action = input.action;

        // Resolve pending motion
        // if let Some(motion) = self.pending_motion.take() {
        //     return self.resolve_motion(handler, motion, input, state);
        // }

        match action.kind() {
            ActionKind::Operator => self.handle_operator(handler, input, None, state),

            ActionKind::Motion => {
                // if action.requires_operand() {
                //     self.pending_motion = Some(action);
                //     return Ok(ActionOutcome::Consumed);
                // }

                if let Some(op) = self.pending_operator.take() {
                    let next = ActionInput {
                        action: op,
                        event: input.event.clone(),
                        repeat: input.repeat,
                    };

                    let (_, range) = handler.handle_motion(input, state)?;

                    self.handle_operator(handler, next, range, state)
                } else {
                    let (status, _) = handler.handle_motion(input, state)?;
                    Ok(status)
                }
            }
            ActionKind::Mode => self.switch_mode(input.action, state),
            ActionKind::Command => handler.handle_command(input, state),
        }
    }

    fn resolve_motion<H: HandleAction>(
        &mut self,
        handler: H,
        motion: Action,
        input: ActionInput,
        state: &mut AppState,
    ) -> ActionResult {
        // Produce the range for the pending motion
        let (status, range) = handler.handle_motion(input.with_action(motion), state)?;

        // Possibly apply it to an active operator
        if let Some(op) = self.pending_operator.take() {
            return handler.handle_operator(input.with_action(op), range, state);
        }

        Ok(status)
    }

    fn handle_visual<H: HandleAction>(
        &mut self,
        handler: H,
        input: ActionInput,
        state: &mut AppState,
    ) -> ActionResult {
        let focus = state.focus;

        match input.action.kind() {
            ActionKind::Motion => {
                let cursor_before = state.cursor();
                let (status, range) = handler.handle_motion(input, state)?;

                if let Some(range) = range {
                    let line_range = constrain_range_to_rule_line(focus, cursor_before, range);

                    if let MotionRange::Single(pos) = line_range {
                        state.mut_selection().update(pos);
                    }
                } else {
                    state.mut_selection().reset();
                }

                Ok(status)
            }

            ActionKind::Operator => {
                let range = state.selection().range();
                self.exit_visual(state);
                self.mode = Mode::Normal;

                self.handle_operator(handler, input, Some(range), state)
            }

            ActionKind::Mode => self.switch_mode(input.action, state),
            ActionKind::Command => handler.handle_command(input, state),
        }
    }

    fn handle_insert<H: HandleAction>(
        &mut self,
        _handler: H,
        _input: ActionInput,
        _state: &mut AppState,
    ) -> ActionResult {
        Ok(ActionOutcome::Consumed)
    }

    fn switch_mode(&mut self, action: Action, state: &mut AppState) -> ActionResult {
        let mode: Mode = action.try_into().map_err(|_| {
            Error::Custom(format!(
                "Invalid action {action:?} from the current mode {:?}",
                self.mode
            ))
        })?;

        match mode {
            Mode::Visual(kind) => {
                let kind = match state.focus {
                    Focus::RulesTop | Focus::RulesLeft => SelectionKind::Cells,
                    _ => kind,
                };

                self.enter_visual(kind, state.cursor(), state);
            }
            _ => {
                self.exit_visual(state);
            }
        }

        self.mode = mode;
        Ok(ActionOutcome::Consumed)
    }

    fn enter_visual(&mut self, kind: SelectionKind, cursor: AppPosition, state: &mut AppState) {
        self.mode = Mode::Visual(kind);

        tracing::debug!("Starting {kind:?} with cursor {cursor:?}");
        state.mut_selection().start(cursor, kind);
    }

    fn exit_visual(&mut self, state: &mut AppState) {
        state.mut_selection().reset();
    }
}

fn constrain_range_to_rule_line(
    focus: Focus,
    cursor_before: AppPosition,
    range: MotionRange,
) -> MotionRange {
    match focus {
        Focus::RulesLeft => match range {
            MotionRange::Block(mut rect) => {
                rect.y = cursor_before.y;
                rect.height = 1;
                MotionRange::Block(rect)
            }
            MotionRange::Rows { .. } => MotionRange::Rows {
                start: cursor_before.y,
                end: cursor_before.y,
            },
            other => other,
        },

        Focus::RulesTop => match range {
            MotionRange::Block(mut rect) => {
                rect.x = cursor_before.x;
                rect.width = 1;
                MotionRange::Block(rect)
            }
            MotionRange::Cols { .. } => MotionRange::Cols {
                start: cursor_before.x,
                end: cursor_before.x,
            },
            other => other,
        },

        _ => range,
    }
}
