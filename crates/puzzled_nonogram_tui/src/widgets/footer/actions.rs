use crossterm::event::{Event, MouseEvent};
use puzzled_nonogram::Fill;
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
                let fill = state.puzzle.fill;
                if matches!(fill, Fill::Color(_)) {
                    let colors = state.puzzle.puzzle.colors();
                    let next = match action {
                        Action::MoveLeft | Action::ScrollLeft => colors.prev(fill),
                        Action::MoveRight | Action::ScrollRight => colors.next(fill),
                        _ => Some(fill),
                    };

                    if let Some(next) = next {
                        state.puzzle.fill = next;
                    }
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
    tracing::info!("Axis region: {:?}", state.footer.order_region);

    // Check if a fill region was clicked
    for region in &mut state.footer.fill_regions {
        if region.area.contains(pos) {
            state.puzzle.fill = region.data;
        }
    }

    // Check if the axis region was clicked
    if state.footer.order_region.area.contains(pos) {
        state.puzzle.motion_order.flip();
    }
}
