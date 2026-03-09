use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Motion<M> {
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

    // Other (for puzzle specific motions)
    #[serde(untagged)]
    Other(M),
}
