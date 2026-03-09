use derive_more::Display;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, Hash, PartialEq, Eq, Display)]
#[serde(rename_all = "snake_case")]
pub enum CrosswordMotion {
    Clue,
}
