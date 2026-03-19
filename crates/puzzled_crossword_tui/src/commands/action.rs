use derive_more::Display;
use puzzled_tui::{ActionBehavior, Describe};
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

impl Describe for CrosswordAction {
    fn describe(&self) -> Option<String> {
        None
    }
}
