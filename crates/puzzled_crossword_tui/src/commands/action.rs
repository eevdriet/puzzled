use derive_more::Display;
use puzzled_tui::ActionBehavior;
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
