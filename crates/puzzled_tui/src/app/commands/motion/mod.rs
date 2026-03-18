mod behavior;
mod handle;
mod range;
mod selection;

pub use behavior::*;
use crossterm::event::MouseEvent;
pub use handle::*;
pub use range::*;
pub use selection::*;

use puzzled_core::Direction;
use serde::Deserialize;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Motion<M> {
    // No motion
    #[default]
    None,

    // Mouse
    #[serde(skip)]
    Mouse(MouseEvent),

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
    Custom(M),
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
