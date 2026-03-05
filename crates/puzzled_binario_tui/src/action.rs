use puzzled_tui::ActionHydrate;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BinarioAction {
    Random,
}

impl ActionHydrate for BinarioAction {
    fn hydrate(self, _event: puzzled_tui::AppEvent, _count: usize) -> Self {
        self
    }
}
