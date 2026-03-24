use derive_more::Display;
use puzzled_core::Grid;
use puzzled_tui::{ActionBehavior, Description, GridRenderState, HandleCustomAction};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, Display, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum BinarioAction {
    Random,
}

impl ActionBehavior for BinarioAction {
    fn variants() -> Vec<Self> {
        vec![BinarioAction::Random]
    }
}

impl Description<()> for BinarioAction {
    fn description(&self, _state: &()) -> Option<String> {
        None
    }
}

impl<T> HandleCustomAction<BinarioAction, GridRenderState, ()> for Grid<T> {
    fn handle_custom_action(
        &mut self,
        _action: BinarioAction,
        _state: &mut GridRenderState,
        _custom_state: &mut (),
    ) -> bool {
        true
    }
}
