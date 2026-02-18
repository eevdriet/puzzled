use crossterm::event::{Event, MouseEvent};
use puzzled_nono::Fill;
use ratatui::layout::Position;

use crate::{
    Action, ActionInput, ActionOutcome, AppState, Error, FooterWidget, HandleAction, MotionRange,
    Result,
};

impl HandleAction for &FooterWidget {
    fn handle_motion(
        &self,
        input: ActionInput,
        state: &mut AppState,
    ) -> Result<(ActionOutcome, Option<MotionRange>)> {
        let action = input.action;
        let event = input.event;

        match action {
            Action::MoveLeft | Action::ScrollLeft | Action::MoveRight | Action::ScrollRight => {
                if let Fill::Color(curr) = state.puzzle.fill {
                    let next = match action {
                        Action::MoveLeft | Action::ScrollLeft => (curr - 1).max(1),
                        Action::MoveRight | Action::ScrollRight => {
                            (curr + 1).min(state.puzzle.style.colors.len() as u16 - 1)
                        }
                        _ => curr,
                    };

                    state.puzzle.fill = Fill::Color(next);
                }
            }
            a if a.is_mouse() => {
                let Event::Mouse(mouse) = *event else {
                    return Err(Error::Custom(format!(
                        "Found invalid event {event:?} for {action:?}"
                    )));
                };

                handle_mouse(mouse, state);
            }

            _ => {}
        }

        Ok((ActionOutcome::Consumed, None))
    }

    fn handle_command(&self, input: ActionInput, _state: &mut AppState) -> crate::ActionResult {
        let action = input.action;

        // Lose focus commands
        if matches!(
            action,
            Action::FocusDown | Action::FocusUp | Action::FocusLeft | Action::FocusRight
        ) {
            tracing::debug!("LOST FOCUS");
            return Ok(ActionOutcome::LoseFocus);
        }

        Ok(ActionOutcome::Consumed)
    }
}

fn handle_mouse(mouse: MouseEvent, state: &mut AppState) {
    let pos = Position::new(mouse.column, mouse.row);
    tracing::info!("Clicked at {pos:?}");
    tracing::info!("Axis region: {:?}", state.footer.axis_region);

    // Check if a fill region was clicked
    for region in &mut state.footer.fill_regions {
        if region.area.contains(pos) {
            state.puzzle.fill = region.data;
        }
    }

    // Check if the axis region was clicked
    if state.footer.axis_region.area.contains(pos) {
        state.puzzle.motion_axis.switch();
    }
}
