use derive_more::Display;
use puzzled_tui::{ActionBehavior, Description};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, Display, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CrosswordAction {
    RevealClue,
}

impl ActionBehavior for CrosswordAction {
    fn variants() -> Vec<Self> {
        vec![CrosswordAction::RevealClue]
    }
}

impl Description<()> for CrosswordAction {
    fn description(&self, _state: &()) -> Option<String> {
        None
    }
}
