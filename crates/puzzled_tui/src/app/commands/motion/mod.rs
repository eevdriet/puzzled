mod behavior;
mod handle;
mod selection;

pub use behavior::*;
use derive_more::Debug;
pub use handle::*;
pub use selection::*;

use crossterm::event::MouseEvent;
use puzzled_core::Direction;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Motion<M> {
    // Mouse
    #[serde(skip)]
    Mouse(#[debug(skip)] MouseEvent),

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

    // Custom (for puzzle specific motions)
    #[serde(untagged)]
    #[debug("{_0:?}")]
    Custom(M),
}

impl<M> Ord for Motion<M>
where
    M: MotionBehavior,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let variants = Self::variants();
        let lhs = variants
            .iter()
            .position(|m| m == self)
            .expect("All variants should be included");
        let rhs = variants
            .iter()
            .position(|m| m == other)
            .expect("All variants should be included");

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

impl<M> TryFrom<&Motion<M>> for Direction {
    type Error = ();

    fn try_from(motion: &Motion<M>) -> Result<Self, Self::Error> {
        let dir = match motion {
            Motion::Down | Motion::ColEnd => Direction::Down,
            Motion::Left | Motion::RowStart => Direction::Left,
            Motion::Right | Motion::RowEnd => Direction::Right,
            Motion::Up | Motion::ColStart => Direction::Up,
            _ => return Err(()),
        };

        Ok(dir)
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
