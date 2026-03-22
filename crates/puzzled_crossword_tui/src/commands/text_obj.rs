use derive_more::{Debug, Display, Eq};
use puzzled_tui::{Description, TextObjectBehavior};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, Display, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum CrosswordTextObject {
    #[debug("Clue")]
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

impl Description<()> for CrosswordTextObject {
    fn description(&self, _state: &()) -> Option<String> {
        let desc = match self {
            CrosswordTextObject::Clue(_) => "Inside a clue",
        };

        Some(desc.to_string())
    }
}
