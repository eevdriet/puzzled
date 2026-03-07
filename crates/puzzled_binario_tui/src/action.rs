use derive_more::Display;
use puzzled_tui::ActionBehavior;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, Display)]
#[serde(rename_all = "snake_case")]
pub enum BinarioAction {
    Random,
}

impl ActionBehavior for BinarioAction {
    fn is_mouse(&self) -> bool {
        false
    }

    fn variants() -> Vec<Self> {
        vec![BinarioAction::Random]
    }
}
