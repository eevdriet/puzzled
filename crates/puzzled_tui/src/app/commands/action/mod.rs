mod behavior;

pub use behavior::*;

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use derive_more::{Display, Eq};
use ratatui::layout::Position as AppPosition;
use serde::Deserialize;

use crate::EventMode;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Display, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Action<A = ()> {
    // Lifetime management
    Quit,
    Select,
    Cancel,

    // Mode management
    #[serde(skip)]
    NextMode(EventMode),

    // -- Normal -- //
    // Mouse
    #[serde(skip)]
    #[display("{pos}")]
    Click {
        #[eq(skip)]
        pos: AppPosition,

        #[eq(skip)]
        button: MouseButton,
    },
    #[serde(skip)]
    #[display("{pos}")]
    Drag {
        #[eq(skip)]
        pos: AppPosition,

        #[eq(skip)]
        button: MouseButton,
    },

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
