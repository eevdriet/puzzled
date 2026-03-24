use derive_more::Display;
use puzzled_core::Grid;
use puzzled_tui::{ActionBehavior, Description, GridRenderState, HandleCustomAction};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, Display, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CrosswordAction {}

impl ActionBehavior for CrosswordAction {
    fn variants() -> Vec<Self> {
        vec![]
    }
}

impl Description<()> for CrosswordAction {
    fn description(&self, _state: &()) -> Option<String> {
        None
    }
}

impl<T> HandleCustomAction<CrosswordAction, GridRenderState, ()> for Grid<T> {
    fn handle_custom_action(
        &mut self,
        _action: CrosswordAction,
        _state: &mut GridRenderState,
        _custom_state: &mut (),
    ) -> bool {
        true
    }
}
