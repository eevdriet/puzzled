use derive_more::Display;
use puzzled_tui::{ActionBehavior, Describe};
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

impl Describe for BinarioAction {
    fn describe(&self) -> Option<String> {
        None
    }
}
