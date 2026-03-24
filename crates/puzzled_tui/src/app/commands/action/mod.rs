mod behavior;
mod description;
mod handle;

use std::cmp::Ordering;

pub use behavior::*;
pub use handle::*;

use crossterm::event::{KeyCode, MouseButton};
use derive_more::Eq;
use ratatui::layout::Position;
use serde::Deserialize;

use crate::SelectionKind;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Action<A> {
    // Lifetime management
    Quit,
    Select,
    Cancel,

    // -- Normal -- //
    // Mouse
    StartSelection {
        pos: Option<Position>,
        kind: SelectionKind,
        additive: bool,
    },
    EndSelection,
    Click(MouseButton),

    // Focus
    FocusDown,
    FocusLeft,
    FocusRight,
    FocusUp,

    // Viewport
    BottomViewport,
    CenterViewport,
    TopViewport,

    // -- Command -- //
    Undo,
    Redo,

    ShowHelp,

    // Literal
    Literal(KeyCode),

    // Other (for puzzle specific actions)
    #[serde(untagged)]
    Custom(A),
}

impl<A> Action<A> {
    fn as_usize(&self) -> usize {
        use Action::*;

        match self {
            Quit => 0,
            Select => 1,
            Cancel => 2,

            // -- Normal -- //
            // Mouse
            StartSelection { .. } => 3,
            EndSelection => 4,
            Click(_) => 4,

            // Focus
            FocusDown => 5,
            FocusLeft => 6,
            FocusRight => 7,
            FocusUp => 8,

            // Viewport
            BottomViewport => 9,
            CenterViewport => 10,
            TopViewport => 11,

            // -- Command -- //
            Undo => 14,
            Redo => 15,

            ShowHelp => 16,

            // Literal
            Literal(..) => 17,

            // Other (for puzzle specific actions)
            Custom { .. } => 18,
        }
    }
}

impl<A> Ord for Action<A>
where
    A: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Action::Literal(lhs), Action::Literal(rhs)) => {
                lhs.partial_cmp(rhs).unwrap_or(Ordering::Less)
            }
            (Action::Custom(lhs), Action::Custom(rhs)) => lhs.cmp(rhs),
            (lhs, rhs) => lhs.as_usize().cmp(&rhs.as_usize()),
        }
    }
}

impl<A> PartialOrd for Action<A>
where
    A: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
