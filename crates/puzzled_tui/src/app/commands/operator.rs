use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    // Text
    Change,
    Delete,
    Yank,

    // Puzzle
    Reveal,
    Check,
}
