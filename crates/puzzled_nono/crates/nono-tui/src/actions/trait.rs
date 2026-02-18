use crate::{ActionInput, ActionOutcome, AppState, MotionRange, Result};

pub type ActionResult = Result<ActionOutcome>;

pub trait HandleAction {
    fn handle_action(&self, _input: ActionInput, _state: &mut AppState) -> ActionResult {
        Ok(ActionOutcome::Consumed)
    }

    fn handle_operator(
        &self,
        _input: ActionInput,
        _range: Option<MotionRange>,
        _state: &mut AppState,
    ) -> ActionResult {
        Ok(ActionOutcome::Consumed)
    }

    fn handle_motion(
        &self,
        _input: ActionInput,
        _state: &mut AppState,
    ) -> Result<(ActionOutcome, Option<MotionRange>)> {
        Ok((ActionOutcome::Consumed, None))
    }

    fn handle_mode(&self, _input: ActionInput, _state: &mut AppState) -> ActionResult {
        Ok(ActionOutcome::Consumed)
    }

    fn handle_command(&self, _input: ActionInput, _state: &mut AppState) -> ActionResult {
        Ok(ActionOutcome::Consumed)
    }
}
