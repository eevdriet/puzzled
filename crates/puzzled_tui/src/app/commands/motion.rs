use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Motion {
    // Left-right
    Col(usize),
    Left,
    Right,
    RowEnd,
    RowStart,

    // Up-down
    ColEnd,
    ColStart,
    Down,
    Row(usize),
    Up,

    // Word
    Word,
}
