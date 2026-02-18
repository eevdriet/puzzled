use crossterm::event::MouseEvent;
use nono::Fill;
use ratatui::layout::Position;

use crate::{Action, ActionInput, ActionOutcome, ActionResult, AppState, Region};

pub fn handle_command(input: ActionInput, _state: &mut AppState) -> ActionResult {
    let action = input.action;

    if matches!(
        action,
        Action::FocusDown | Action::FocusUp | Action::FocusLeft | Action::FocusRight
    ) {
        tracing::debug!("LOST RULES FOCUS");
        return Ok(ActionOutcome::LoseFocus);
    }

    Ok(ActionOutcome::Consumed)
}

pub fn handle_mouse(regions: &Vec<Region<Fill>>, mouse: MouseEvent, fill: &mut Fill) -> bool {
    let pos = Position::new(mouse.column, mouse.row);

    for region in regions {
        if region.area.contains(pos) {
            *fill = region.data;
            return true;
        }
    }

    false
}
