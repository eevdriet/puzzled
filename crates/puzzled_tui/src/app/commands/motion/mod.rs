mod behavior;
mod description;
mod handle;
mod search;
mod selection;

pub use behavior::*;
pub use handle::*;
pub use search::*;
pub use selection::*;

use crossterm::event::MouseEvent;
use derive_more::Debug;
use puzzled_core::Direction;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Motion<M> {
    // Mouse
    #[serde(skip)]
    Mouse(#[debug(skip)] MouseEvent),

    // Left-right
    #[debug("Col(n)")]
    Col(#[serde(skip, default)] usize),
    Left,
    Right,
    RowEnd,
    RowStart,

    // Up-down
    ColEnd,
    ColStart,
    Down,

    #[debug("Row(n)")]
    Row(#[serde(skip, default)] usize),
    Up,

    // General
    Forwards,
    Backwards,

    WordEndBackwards,
    WordEndForwards,
    WordStartBackwards,
    WordStartForwards,

    // Custom (for puzzle specific motions)
    #[serde(untagged)]
    #[debug("{_0:?}")]
    Custom(M),

    // Word
    #[serde(untagged)]
    #[debug("{_0:?}")]
    Search(SearchMotion),
}

impl<M> Motion<M> {
    pub fn is_search(&self) -> bool {
        matches!(self, Motion::Search(_))
    }
}

impl<M> Ord for Motion<M>
where
    M: MotionBehavior,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let variants = Self::variants();

        let lhs = variants.iter().position(|m| m == self).unwrap_or_else(|| {
            panic!("Unfound motion: {self:?} (variants: {variants:?})");
        });

        let rhs = variants.iter().position(|m| m == other).unwrap_or_else(|| {
            panic!("Unfound motion: {other:?} (variants: {variants:?})");
        });

        lhs.cmp(&rhs)
    }
}

impl<M> PartialOrd for Motion<M>
where
    M: MotionBehavior,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<M> From<Direction> for Motion<M> {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Down => Motion::Down,
            Direction::Left => Motion::Left,
            Direction::Right => Motion::Right,
            Direction::Up => Motion::Up,
        }
    }
}
