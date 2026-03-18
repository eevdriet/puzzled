mod behavior;
mod handle;

pub use behavior::*;
pub use handle::*;

use crossterm::event::MouseButton;
use derive_more::{Display, Eq};
use ratatui::layout::Position as AppPosition;
use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Display, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Action<A> {
    // Lifetime management
    Quit,
    Select,
    Cancel,

    // -- Normal -- //

    // Focus
    FocusDown,
    FocusLeft,
    FocusRight,
    FocusUp,

    // Viewport
    BottomViewport,
    CenterViewport,
    TopViewport,

    // -- Insert/Replace -- //
    Insert(char),
    Replace(char),
    DeleteLeft,
    DeleteRight,

    // -- Command -- //
    Undo,
    Redo,

    // Other (for puzzle specific actions)
    #[serde(untagged)]
    Other(A),
}
