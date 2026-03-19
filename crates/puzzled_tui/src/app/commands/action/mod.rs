mod behavior;
mod handle;

pub use behavior::*;
pub use handle::*;

use derive_more::{Display, Eq};
use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Display, Hash, PartialOrd, Ord)]
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

    ShowHelp,

    // Other (for puzzle specific actions)
    #[serde(untagged)]
    Other(A),
}
