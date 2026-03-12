use derive_more::{Display, Eq};
use puzzled_tui::TextObjectBehavior;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, Display)]
#[serde(rename_all = "snake_case")]
pub enum CrosswordTextObject {
    Clue(
        #[serde(skip, default)]
        #[eq(skip)]
        u8,
    ),
}

impl TextObjectBehavior for CrosswordTextObject {
    fn variants() -> Vec<Self> {
        vec![Self::Clue(0)]
    }
}
