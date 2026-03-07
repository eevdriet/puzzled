mod behavior;
mod handle;
mod resolver;

use std::hash::Hash;

pub use behavior::*;
pub use handle::*;
pub use resolver::*;

use derive_more::{Display, Eq};
use ratatui::layout::Position;
use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum Action<A = ()> {
    // Lifetime management
    Quit,
    Select,
    Cancel,

    // Mouse
    Click(
        #[serde(skip, default)]
        #[eq(skip)]
        Position,
    ),
    Drag(
        #[serde(skip, default)]
        #[eq(skip)]
        Position,
    ),
    ScrollDown(
        #[serde(skip, default)]
        #[eq(skip)]
        Position,
    ),
    ScrollLeft(
        #[serde(skip, default)]
        #[eq(skip)]
        Position,
    ),
    ScrollRight(
        #[serde(skip, default)]
        #[eq(skip)]
        Position,
    ),
    ScrollUp(
        #[serde(skip, default)]
        #[eq(skip)]
        Position,
    ),

    // Focus
    FocusDown,
    FocusLeft,
    FocusRight,
    FocusUp,

    // Movement
    MoveDown(
        #[serde(skip, default = "default_count")]
        #[eq(skip)]
        usize,
    ),
    MoveLeft(
        #[serde(skip, default = "default_count")]
        #[eq(skip)]
        usize,
    ),
    MoveRight(
        #[serde(skip, default = "default_count")]
        #[eq(skip)]
        usize,
    ),
    MoveUp(
        #[serde(skip, default = "default_count")]
        #[eq(skip)]
        usize,
    ),

    MoveRow(
        #[serde(skip, default)]
        #[eq(skip)]
        usize,
    ),
    MoveRowStart,
    MoveRowEnd,

    MoveCol(
        #[serde(skip, default)]
        #[eq(skip)]
        usize,
    ),
    MoveColStart,
    MoveColEnd,

    // Solving
    Reveal,
    RevealAll,

    // Viewport
    BottomViewport,
    CenterViewport,
    TopViewport,

    // Commands
    Undo,
    Redo,

    // Other (for puzzle specific actions)
    #[serde(untagged)]
    Other(A),
}

fn default_count() -> usize {
    1
}

impl<A> Hash for Action<A>
where
    A: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {}
}
